#[cfg(test)]
mod tests {
    use crate::assemble;
    use std::path::PathBuf;

    macro_rules! assert_stdout_eq {
        ($test:expr, $expected:literal) => {{
            use gag::BufferRedirect;
            use std::io::Read;

            let mut buf = BufferRedirect::stdout().unwrap();

            $test;

            let mut output = String::new();
            buf.read_to_string(&mut output).unwrap();
            drop(buf);

            assert_eq!(&output[..], $expected);
        }};
    }

    #[test]
    fn test_0_output() {
        assert_stdout_eq!(assemble(PathBuf::from("test/test_0_output.s")), "\n[0]: 10110001 10110010 10110100 10111000 \n[4]: 10110001 10110010 10110100 10111000 \n[8]: 10110001 10110010 10110100 10111000 \n[c]: 10110001 10110010 10110100 10111000 ");
    }

    #[test]
    fn test_1_jmp() {
        assert_stdout_eq!(
            assemble(PathBuf::from("test/test_1_jmp.s")),
            "\n[0]: 10110001 10110010 10110100 11110000 "
        );
    }

    #[test]
    fn test_2_input() {
        assert_stdout_eq!(
            assemble(PathBuf::from("test/test_2_input.s")),
            "\n[0]: 01100000 01010001 10010000 11110001 "
        );
    }

    #[test]
    fn test_3_alu() {
        assert_stdout_eq!(assemble(PathBuf::from("test/test_3_alu.s")), "\n[0]: 10111111 01110001 01011111 11100000 \n[4]: 10110001 10110010 10110100 10111000 \n[8]: 10110001 10110010 10110100 10111000 \n[c]: 10110001 10110010 10110100 10111000 ");
    }

    #[test]
    fn test_4_a() {
        assert_stdout_eq!(assemble(PathBuf::from("test/test_4_a.s")), "\n[0]: 00100000 00000001 01000000 10010000 \n[4]: 11110001 ");
    }

    #[test]
    fn test_5_invert() {
        assert_stdout_eq!(assemble(PathBuf::from("test/test_5_invert.s")), "\n[0]: 00111111 01100000 00000001 01010001 \n[4]: 11100010 01000000 10010000 11110111 ");
    }
}
