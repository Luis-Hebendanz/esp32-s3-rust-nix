# ESP-RS Development Environment Template


# Getting Started with the Development Environment

Let's get your development environment up and running:

1. **Install Nix Package Manager**:

   - You can install the Nix package manager by either [downloading the Nix installer](https://github.com/DeterminateSystems/nix-installer/releases) or running this command:
     ```bash
     curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install
     ```

On Windows Subsystem for Linux (WSL) the installer will fail and tell you what to do. Execute the command from the error message and then afterwards execute:

```bash
sudo echo "experimental-features = nix-command flakes" > '/etc/nix/nix.conf'
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

6. **Build the Project**
    - Go the nes-vcr directory and execute:
    ```bash
    cargo build
    ```
    - Or to build and flash directly execute:
    ```
    cargo run
    ```

7. **Open up the Documentation**
    - To open the library documentation execute:
    ```bash
    cargo doc --open
    ```
