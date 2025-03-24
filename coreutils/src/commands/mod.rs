macro_rules! command {
    ($mod:ident::$cmd:ident) => {
        mod $mod;
        pub use $mod::$cmd;
    };
}

command!(hello::Hello);
command!(cat::Cat);
command!(echo::Echo);
command!(mkdir::Mkdir);
command!(dd::Dd);
command!(nproc::Nproc);
command!(r#true::True);
command!(r#false::False);
command!(test::Test);
command!(yes::Yes);
command!(pwd::Pwd);
