use axtask::TaskExtRef;
use axhal::mem::VirtAddr;
use axhal::paging::MappingFlags;
use axhal::trap::{register_trap_handler, PAGE_FAULT};
//use axhal::paging::{MappingFlags,VirtAddr};
///pub static PAGE_FAULT: [fn(VirtAddr, MappingFlags, bool) -> bool];
#[register_trap_handler(PAGE_FAULT)]
fn page_fault_handle(vaddr:VirtAddr, access_flags:MappingFlags, user:bool) -> bool {
    ax_println!("handle ph ...");
    if user{//用户态。。。
        if !axtask::current().task_ext().aspace.lock().handle_page_fault(vaddr, access_flags){
            //获取当前任务              .lock user mem try handle
            ax_println!("failed,exit....");
            axtask::exit(-1);
            //return true;
        }else{
            ax_println!("handle success");
            
        }
        return true;
    }
    //内核态
    false
}

