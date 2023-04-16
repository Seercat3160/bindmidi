pub mod gui_macros {
    //! Some macros to make it easier to work with the GUI crate without huge amounts of boilerplate

    /// Shadow the given variables with their clones
    #[macro_export]
    macro_rules! shadow_clone {
        ( $( $x:ident ),*) => {
            $(
                let $x = $x.clone();
            )*
        }
    }

    /// Shadow the given variables with their clones, defined as `mut`
    #[macro_export]
    macro_rules! shadow_clone_mut {
        ( $( $x:ident ),*) => {
            $(
                #[allow(unused_mut)]
                let mut $x = $x.clone();
            )*
        }
    }

    /// Show the given libui controls if the expression matches the given patterns
    #[macro_export]
    macro_rules! show_control_only_when {
        ( $d:expr, $( $i:ident : $p:pat ),+ ) => {
            $(
                match $d {
                    $p => $i.show(),
                    _ => $i.hide()
                }
            )+
        }
    }

    /// Enable the given libui controls if the expression matches the given patterns
    #[macro_export]
    macro_rules! enable_control_only_when {
            ( $d:expr, $( $i:ident ),+ ) => {
                $(
                    match $d {
                        true => $i.enable(),
                        false => $i.disable()
                    }
                )+
            }
        }
}
