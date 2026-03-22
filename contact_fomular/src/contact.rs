pub fn welcome_message() {
    println!(
        r"   ___            _             _       ___                          _               ___   ___   _ "
    );
    println!(
        r"  / __\___  _ __ | |_ __ _  ___| |_    / __\__  _ __ _ __ ___  _   _| | __ _ _ __   / _ \ / _ \ / |"
    );
    println!(
        r" / /  / _ \| '_ \| __/ _` |/ __| __|  / _\/ _ \| '__| '_ ` _ \| | | | |/ _` | '__| | | | | | | || |"
    );
    println!(
        r"/ /__| (_) | | | | || (_| | (__| |_  / / | (_) | |  | | | | | | |_| | | (_| | |    | |_| | |_| || |"
    );
    println!(
        r"\____/\___/|_| |_|\__\__,_|\___\___| \/   \___/|_|  |_| |_| |_|\__,_|_|\__,_|_|     \___(_)___(_)_|"
    );
    println!();

    println!("\nWelcome to the contact formular v0.0.1");
    println!("Make sure to answer all question to complete the contact formular.");
}

pub fn bye_message() {
    println!("You did it! You completed the contact formular.");
    println!("Execute exit...");
}
