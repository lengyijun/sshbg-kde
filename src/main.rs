use std::env;
use std::process::Command;

fn main() {
    match env::args().skip(1).next() {
        None => {}
        Some(ssh_hostname) => {
            let mut profile:String = "WhiteOnBlack".to_string();
            if ssh_hostname=="gpu"{
                profile = "BlackOnLightYellow".to_string()
            }else if ssh_hostname=="mpc_blackpool"{
                profile = "BlackOnRandomLight".to_string()
            }
            profile="colors=".to_string()+&profile;
            Command::new("konsoleprofile")
                .arg(&profile)
                .spawn()
                .expect("konsoleprofile command failed to start");
        }
    }
}

/*
* /usr/share/konsole
BlackOnLightYellow.colorscheme
BlackOnRandomLight.colorscheme
BlackOnWhite.colorscheme
BlueOnBlack.colorscheme
Breath2.colorscheme
Breath2-light.colorscheme
Breath2-silverfox.colorscheme
Breeze.colorscheme
DarkPastels.colorscheme
GreenOnBlack.colorscheme
Linux.colorscheme
RedOnBlack.colorscheme
Solarized.colorscheme
SolarizedLight.colorscheme
WhiteOnBlack.colorscheme
*/
