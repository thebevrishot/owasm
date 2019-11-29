#[macro_export]
macro_rules! decl_data {
    (pub struct $data_name:ident {
        $(pub $field_name:ident : $field_type:ty = $field_howto:expr ,)*
    }) => {
        use $crate::core::{Oracle, ShellCmd, execute_with_local_env};

        #[derive(Debug)]
        pub struct $data_name {
            $(pub $field_name : $field_type,)*
        }

        impl $data_name {
            pub fn __input() -> Vec<ShellCmd> {
                vec![ $($field_howto.as_cmd(),)* ]
            }

            pub fn __output(mut output: Vec<String>) -> Option<$data_name> {
                Some($data_name {
                    $($field_name : $field_howto.from_cmd_output(output.remove(0))?,)*
                })
            }

            pub fn build_from_local_env() -> Option< $data_name > {
                Self::__output(execute_with_local_env(Self::__input()))
            }
        }

        pub type __Data = $data_name;
    };
}
