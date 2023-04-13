### RS-Token-Recommendations
RS-Token-Recommendations is a Rust-based command-line utility that scans Svelte, CSS, and SCSS files in a specified directory and its subdirectories to detect hard-coded pixel values. It then recommends the nearest spacing tokens from a predefined list.

This is the Rust version of my original project that I wrote in TypeScript/Node.js.

### Usage
To use this program, download the appropriate binary for your operating system (Windows or Unix) from the GitHub releases.

## Windows
1. Download the Windows executable rs-token-recommendations.exe from the GitHub releases page.
2. Open the Command Prompt.
3. Navigate to the directory containing the downloaded executable.
4. Run the program with the following command:
```
rs-token-recommendations.exe "path/to/your/project"
```
Replace "path/to/your/project" with the path to the directory you want to scan.

## Unix
1. Download the Unix binary rs-token-recommendations from the GitHub releases page.
2. Open the terminal.
3. Navigate to the directory containing the downloaded binary.
4. If necessary, grant execute permissions to the binary:
```
chmod +x rs-token-recommendations
```
5. Run the program with the following command:
```
./rs-token-recommendations "path/to/your/project"
```
Replace "path/to/your/project" with the path to the directory you want to scan.

The program will scan the specified directory and output the detected hard-coded pixel values, along with the recommended tokens, to the console. The output will also be saved to an output.txt file in the current working directory.

If you want to build the project from source or contribute to it, please refer to this GitHub repository.

### Example Output
![image](https://user-images.githubusercontent.com/5696449/231904988-87241279-53e4-4869-9154-271cb4bed1cf.png)
