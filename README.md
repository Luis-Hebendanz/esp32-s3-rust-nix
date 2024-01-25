# ESP32-S3 Rust Development Environment

This is a development environment for the ESP32-s3 board build with nix. It needs the nix.settings.sandbox = "relaxed";
Setting to be toggled as it downloads artifacts at run time from the web.


# Getting Started with the Development Environment

Let's get your development environment up and running:

1. **Install Nix Package Manager**:

   - You can install the Nix package manager by either [downloading the Nix installer](https://github.com/DeterminateSystems/nix-installer/releases) or running this command:
     ```bash
     curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install
     ```

   Afterwards execute the command:
   ```bash
   sudo echo "sandbox = relaxed" >> /etc/nix/nix.conf && sudo systemctl restart nix-daemon.service
   ```


   On Windows Subsystem for Linux (WSL) the installer will fail and tell you what to do. Execute the command from the error message and then afterwards execute:

   ```bash
   sudo echo "experimental-features = nix-command flakes" >> '/etc/nix/nix.conf'
   ```

2. **Install direnv**:

   - Download the direnv package from [here](https://direnv.net/docs/installation.html) or run the following command:
     ```bash
     curl -sfL https://direnv.net/install.sh | bash
     ```

3. **Add direnv to your shell**:

   - Direnv needs to [hook into your shell](https://direnv.net/docs/hook.html) to work.
     You can do this by executing following command:

   ```bash
   echo 'eval "$(direnv hook zsh)"' >> ~/.zshrc && echo 'eval "$(direnv hook bash)"' >> ~/.bashrc && eval "$SHELL"
   ```

4. **Clone the Repository and Navigate**:

   - Clone this repository and navigate to it.
   - If you are under Windows Subystem For Linux (WSL) please clone the repository to the home folder of your Linux. Do NOT clone it onto your Windows machine!

5. **Allow .envrc**:

   - When you enter the directory, you'll receive an error message like this:
     ```bash
     direnv: error .envrc is blocked. Run `direnv allow` to approve its content
     ```
   - Execute `direnv allow` to automatically execute the shell script `.envrc` when entering the directory.

6. **Set Environment Variables**

   - The rust program tries at compile time to read the two environment variables WIFI_SSID and WIFI_PASS
   - Make sure to export them, else you get a compile error. Also start your code editor after having exported these variables
   else the editor will display these error messages regardless.

   ```bash
   export WIFI_SSID="<Your wifi ssid>"
   export WIFI_PASS="<Your wifi password>"
   ```

7. **Open VSCode Workspace**
   - There is a VSCode workspace you can open it with
   ```bash
   code esp32-s3-rust-project.code-workspace
   ```

8. **Build the Project**
    - Go the nes-vcr directory and execute:
    ```bash
    cargo build
    ```
    - Or to build and flash directly execute:
    ```
    cargo run
    ```

9. **Open up the Documentation**
    - To open the library documentation execute:
    ```bash
    cargo doc --open --package esp-idf-svc
    ```

10. **Board Configuration**
   - To configure the board, the `sdkconfig.defaults` file needs to be modified.
   - Go to the reference manual https://docs.espressif.com/projects/esp-idf/en/latest/esp32s3/api-reference/kconfig.html and set the options manually in `sdkconfig.defaults`
   - Or execute `idf.py menuconfig` in another C esp32-s3 project set the options you want and then copy them over to the `sdkconfig.defaults` file
