use crate::{halt, print, println};
use alloc::{format, string::String, vec, vec::Vec};
use core::fmt::Debug;

pub fn test_runner(tests: &'static [&(dyn Testable + Sync)]) {
    println!("\nrunning {} tests", tests.len());

    let mut num_passed = 0;
    let mut num_failed = 0;

    let mut fail_msgs: Vec<(&str, TestErr)> = vec![];

    for test in tests {
        let name = test.name();
        print!("test {name} ... ");
        let result = test.run();
        match result {
            Ok(_) => {
                num_passed += 1;
                println!("\x1B[32mok\x1B[0m");
            }
            Err(msg) => {
                num_failed += 1;
                println!("\x1B[31mFAILED\x1B[0m");
                fail_msgs.push((name, msg));
            }
        }
    }

    if num_failed > 0 {
        if fail_msgs.iter().any(|(_, msg)| msg.is_some()) {
            println!("\nfailures:\n");
            for (name, msg) in &fail_msgs {
                if let Some(msg) = msg {
                    println!("---- {name} output ----");
                    println!("{msg}");
                    println!();
                }
            }
        }

        println!("\nfailures:");
        for (name, _) in &fail_msgs {
            println!("    {name}");
        }
    }

    println!(
        "\ntest result: {}. {} passed; {} failed\n",
        match num_failed {
            0 => "\x1B[32mok\x1B[0m",
            _ => "\x1B[31mFAILED\x1B[0m",
        },
        num_passed,
        num_failed,
    );

    exit(match num_failed {
        0 => ExitCode::Success,
        _ => ExitCode::Failed,
    });
}

fn exit(exit_code: ExitCode) -> ! {
    use x86_64::instructions::port::Port;

    let mut port = Port::new(0xf4);
    unsafe {
        port.write(exit_code as u32);
    }

    halt()
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("\x1b[31mPANICKED\x1b[0m");
    println!();
    println!("{info}");
    exit(ExitCode::Failed)
}

#[repr(u32)]
enum ExitCode {
    Success = 0x10,
    Failed = 0x11,
}

type TestResult = Result<(), TestErr>;
type TestErr = Option<String>;

pub trait TestOutput {
    fn get(&self) -> TestResult;
}

impl TestOutput for () {
    fn get(&self) -> TestResult {
        Ok(())
    }
}

impl TestOutput for bool {
    fn get(&self) -> TestResult {
        match self {
            true => Ok(()),
            false => Err(None),
        }
    }
}

impl<T> TestOutput for Option<T> {
    fn get(&self) -> TestResult {
        match self {
            Some(_) => Ok(()),
            None => Err(None),
        }
    }
}

impl<T, E> TestOutput for Result<T, E>
where
    E: Debug,
{
    fn get(&self) -> TestResult {
        match self {
            Ok(_) => Ok(()),
            Err(msg) => Err(Some(format!("{:#?}", msg))),
        }
    }
}

pub trait Testable {
    fn run(&self) -> TestResult;
    fn name(&self) -> &'static str;
}

impl<F, T> Testable for F
where
    F: Fn() -> T,
    T: TestOutput,
{
    fn run(&self) -> TestResult {
        self().get()
    }

    fn name(&self) -> &'static str {
        core::any::type_name::<F>()
    }
}
