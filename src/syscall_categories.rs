use crate::types::Category;
use std::collections::HashMap;
use syscalls::Sysno;

pub fn initialize_categories_map() -> HashMap<Sysno, Category> {
    use Category::*;
    let array: Vec<(Sysno, Category)> = vec![
        (Sysno::read, DiskIO),
        (Sysno::write, DiskIO),
        (Sysno::pread64, DiskIO),
        (Sysno::pwrite64, DiskIO),
        (Sysno::readv, DiskIO),
        (Sysno::writev, DiskIO),
        (Sysno::preadv, DiskIO),
        (Sysno::pwritev, DiskIO),
        (Sysno::preadv2, DiskIO),
        (Sysno::pwritev2, DiskIO),
        (Sysno::pipe, Process),
        (Sysno::pipe2, Process),
        (Sysno::dup, FileOp),
        (Sysno::dup2, FileOp),
        (Sysno::dup3, FileOp),
        (Sysno::access, FileOp),
        (Sysno::faccessat, FileOp),
        (Sysno::faccessat2, FileOp),
        (Sysno::open, FileOp),
        (Sysno::openat, FileOp),
        (Sysno::openat2, FileOp),
        (Sysno::creat, FileOp),
        (Sysno::getcwd, FileOp),
        (Sysno::chdir, FileOp),
        (Sysno::fchdir, FileOp),
        (Sysno::rename, FileOp),
        (Sysno::renameat, FileOp),
        (Sysno::renameat2, FileOp),
        (Sysno::mkdir, FileOp),
        (Sysno::mkdirat, FileOp),
        (Sysno::link, FileOp),
        (Sysno::linkat, FileOp),
        (Sysno::unlink, FileOp),
        (Sysno::unlinkat, FileOp),
        (Sysno::rmdir, FileOp),
        (Sysno::symlink, FileOp),
        (Sysno::symlinkat, FileOp),
        (Sysno::readlink, FileOp),
        (Sysno::readlinkat, FileOp),
        (Sysno::chmod, FileOp),
        (Sysno::fchmod, FileOp),
        (Sysno::fchmodat, FileOp),
        (Sysno::chown, FileOp),
        (Sysno::fchown, FileOp),
        (Sysno::lchown, FileOp),
        (Sysno::fchownat, FileOp),
        (Sysno::sync, DiskIO),
        (Sysno::syncfs, DiskIO),
        (Sysno::fsync, DiskIO),
        (Sysno::fdatasync, DiskIO),
        (Sysno::truncate, DiskIO),
        (Sysno::ftruncate, DiskIO),
        (Sysno::close, FileOp),
        (Sysno::stat, FileOp),
        (Sysno::fstat, FileOp),
        (Sysno::lstat, FileOp),
        (Sysno::newfstatat, FileOp),
        (Sysno::statx, FileOp),
        (Sysno::statfs, FileOp),
        (Sysno::fstatfs, FileOp),
        (Sysno::ustat, Device),
        (Sysno::cachestat, Memory),
        (Sysno::lseek, DiskIO),
        (Sysno::mmap, Memory),
        (Sysno::mprotect, Memory),
        (Sysno::munmap, Memory),
        (Sysno::brk, Memory),
        (Sysno::mlock, Memory),
        (Sysno::mlock2, Memory),
        (Sysno::munlock, Memory),
        (Sysno::mlockall, Memory),
        (Sysno::munlockall, Memory),
        (Sysno::mremap, Memory),
        (Sysno::msync, Memory),
        (Sysno::mincore, Memory),
        (Sysno::madvise, Memory),
        (Sysno::select, AsyncIO),
        (Sysno::pselect6, AsyncIO),
        (Sysno::poll, AsyncIO),
        (Sysno::ppoll, AsyncIO),
        (Sysno::epoll_create, AsyncIO),
        (Sysno::epoll_create1, AsyncIO),
        (Sysno::epoll_wait, AsyncIO),
        (Sysno::epoll_pwait, AsyncIO),
        (Sysno::epoll_pwait2, AsyncIO),
        (Sysno::epoll_ctl, AsyncIO),
        (Sysno::socket, Network),
        (Sysno::bind, Network),
        (Sysno::getsockname, Network),
        (Sysno::getpeername, Network),
        (Sysno::socketpair, Network),
        (Sysno::setsockopt, Network),
        (Sysno::getsockopt, Network),
        (Sysno::listen, Network),
        (Sysno::accept, Network),
        (Sysno::accept4, Network),
        (Sysno::connect, Network),
        (Sysno::sendto, Network),
        (Sysno::sendmsg, Network),
        (Sysno::recvfrom, Network),
        (Sysno::recvmsg, Network),
        (Sysno::shutdown, Process),
        (Sysno::fcntl, FileOp),
        (Sysno::ioctl, Device),
        (Sysno::arch_prctl, Process),
        (Sysno::sched_yield, Process),
        (Sysno::rt_sigaction, Signals),
        (Sysno::rt_sigprocmask, Signals),
        (Sysno::rt_sigsuspend, Signals),
        (Sysno::sigaltstack, Signals),
        (Sysno::rt_sigreturn, Signals),
        (Sysno::rt_sigpending, Signals),
        (Sysno::rt_sigtimedwait, Signals),
        (Sysno::rt_sigqueueinfo, Signals),
        (Sysno::rt_tgsigqueueinfo, Signals),
        (Sysno::signalfd, Signals),
        (Sysno::signalfd4, Signals),
        (Sysno::pidfd_send_signal, Signals),
        (Sysno::gettid, Thread),
        (Sysno::getpid, Thread),
        (Sysno::getppid, Thread),
        (Sysno::getrandom, Device),
        (Sysno::setrlimit, Process),
        (Sysno::getrlimit, Process),
        (Sysno::prlimit64, Process),
        (Sysno::getrusage, Process),
        (Sysno::sysinfo, Process),
        (Sysno::times, Process),
        (Sysno::sched_setaffinity, CPU),
        (Sysno::sched_getaffinity, CPU),
        (Sysno::exit, Process),
        (Sysno::exit_group, Process),
        (Sysno::kill, Signals),
        (Sysno::tgkill, Signals),
        (Sysno::tkill, Signals),
        (Sysno::pause, Signals),
        (Sysno::ptrace, System),
        (Sysno::rseq, Thread),
        (Sysno::uname, System),
        (Sysno::getuid, Process),
        (Sysno::geteuid, Process),
        (Sysno::getgid, Process),
        (Sysno::getegid, Process),
        (Sysno::setuid, Process),
        (Sysno::setgid, Process),
        (Sysno::futex, AsyncIO),
        (Sysno::set_tid_address, Thread),
        (Sysno::eventfd, FileOp),
        (Sysno::eventfd2, FileOp),
        (Sysno::wait4, Process),
        (Sysno::waitid, Process),
        (Sysno::set_robust_list, Process),
        (Sysno::get_robust_list, Process),
        (Sysno::setpgid, Process),
        (Sysno::getpgid, Process),
        (Sysno::getpgrp, Process),
        (Sysno::fork, Process),
        (Sysno::vfork, Process),
        (Sysno::clone3, Process),
        (Sysno::clone, Process),
        (Sysno::nanosleep, Process),
        (Sysno::execve, Process),
        (Sysno::landlock_create_ruleset, Security),
        (Sysno::landlock_add_rule, Security),
        (Sysno::landlock_restrict_self, Security),
        (Sysno::fallocate, DiskIO),
        (Sysno::getpriority, Process),
        (Sysno::setpriority, Process),
        (Sysno::getdents, DiskIO),
        (Sysno::getdents64, DiskIO),
    ];
    array.into_iter().collect()
}
