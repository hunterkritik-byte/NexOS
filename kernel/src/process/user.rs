use crate::x86_kernel::process::elf::{parse_elf64, program_headers, PT_LOAD};
use x86_64::{
    VirtAddr,
    structures::paging::{
        FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PageTableFlags, PhysFrame,
        Size4KiB,
    },
};

pub const USER_STACK_TOP: u64 = 0x0080_0000;
pub const USER_STACK_PAGES: u64 = 8;

pub struct UserProcess {
    pub root: PhysFrame<Size4KiB>,
    pub entry: u64,
    pub stack_top: u64,
}

unsafe fn copy_to_frame(
    physical_offset: VirtAddr,
    frame: PhysFrame<Size4KiB>,
    page_offset: usize,
    data: &[u8],
) {
    let dst = (physical_offset + frame.start_address().as_u64() + page_offset as u64)
        .as_mut_ptr::<u8>();
    core::ptr::copy_nonoverlapping(data.as_ptr(), dst, data.len());
}

pub unsafe fn build_elf_process<A>(
    physical_offset: VirtAddr,
    active: &PageTable,
    allocator: &mut A,
    image: &[u8],
) -> Result<UserProcess, &'static str>
where
    A: FrameAllocator<Size4KiB>,
{
    let header = parse_elf64(image)?;
    if header.entry >= 0x0000_8000_0000_0000 {
        return Err("ELF entry outside canonical user range");
    }
    let (headers, count) = program_headers(image, header)?;
    if count == 0 {
        return Err("ELF contains no PT_LOAD segments");
    }

    let root = allocator.allocate_frame().ok_or("out of physical memory")?;
    let root_virt = physical_offset + root.start_address().as_u64();
    let root_table = &mut *root_virt.as_mut_ptr::<PageTable>();
    root_table.zero();

    // Keep only the kernel half of the active address space.
    for index in 256..512 {
        root_table[index] = active[index].clone();
    }

    let mut mapper = OffsetPageTable::new(root_table, physical_offset);
    let user = PageTableFlags::PRESENT | PageTableFlags::USER_ACCESSIBLE;

    for ph in headers[..count].iter().flatten() {
        if ph.vaddr < 0x1000 || ph.vaddr.checked_add(ph.memsz).ok_or("ELF address overflow")?
            >= 0x0000_8000_0000_0000
        {
            return Err("ELF segment outside user address space");
        }

        let seg_start = ph.vaddr & !0xfff;
        let seg_end = ph.vaddr.checked_add(ph.memsz).ok_or("ELF segment overflow")?;
        let first_page = Page::containing_address(VirtAddr::new(seg_start));
        let last_page = Page::containing_address(VirtAddr::new(seg_end.saturating_sub(1) & !0xfff));

        let mut flags = user;
        if ph.flags & 2 != 0 {
            flags |= PageTableFlags::WRITABLE;
        }
        if ph.flags & 1 == 0 {
            flags |= PageTableFlags::NO_EXECUTE;
        }

        for page in Page::range_inclusive(first_page, last_page) {
            let frame = allocator.allocate_frame().ok_or("out of physical memory")?;
            mapper.map_to(page, frame, flags, allocator).map_err(|_| "map ELF segment")?.flush();

            // Zero the complete page before copying file-backed bytes.
            let dst = (physical_offset + frame.start_address().as_u64()).as_mut_ptr::<u8>();
            core::ptr::write_bytes(dst, 0, 4096);

            let page_start = page.start_address().as_u64();
            let copy_start = core::cmp::max(page_start, ph.vaddr);
            let copy_end = core::cmp::min(page_start + 4096, ph.vaddr + ph.filesz);
            if copy_end > copy_start {
                let image_offset = ph.offset + (copy_start - ph.vaddr);
                let len = (copy_end - copy_start) as usize;
                let src_end = image_offset.checked_add(len as u64).ok_or("ELF file overflow")?;
                if src_end > image.len() as u64 {
                    return Err("ELF segment exceeds file");
                }
                copy_to_frame(
                    physical_offset,
                    frame,
                    (copy_start - page_start) as usize,
                    &image[image_offset as usize..src_end as usize],
                );
            }
        }
    }

    // Allocate a private user stack.
    for i in 0..USER_STACK_PAGES {
        let addr = USER_STACK_TOP - (i + 1) * 4096;
        let frame = allocator.allocate_frame().ok_or("out of physical memory")?;
        let page = Page::containing_address(VirtAddr::new(addr));
        mapper
            .map_to(
                page,
                frame,
                user | PageTableFlags::WRITABLE | PageTableFlags::NO_EXECUTE,
                allocator,
            )
            .map_err(|_| "map user stack")?
            .flush();
        let dst = (physical_offset + frame.start_address().as_u64()).as_mut_ptr::<u8>();
        core::ptr::write_bytes(dst, 0, 4096);
    }

    Ok(UserProcess {
        root,
        entry: header.entry,
        stack_top: USER_STACK_TOP - 16,
    })
}

pub unsafe fn install_user_cr3(root: PhysFrame<Size4KiB>) {
    core::arch::asm!(
        "mov cr3, {}",
        in(reg) root.start_address().as_u64(),
        options(nostack, preserves_flags)
    );
}

pub unsafe fn launch_user_process(process: UserProcess) -> ! {
    let context = super::context::UserContext::new(process.entry, process.stack_top);
    install_user_cr3(process.root);
    super::context::enter_user(&context);
}
