# Qictex
Qictex is a language that gets compiled directly to a .tex file. it aims to drastically speed up the process of digitally writing down your mathematical texts by removing unnecessary overhead (e.g. \begin{document}) and by shortening commonly used symbols (e.g. "\cdot" => "*", "\Leftrightarrow" => "<=>").

## Table of contents
 - [Installation](#installation)
 - [Usage](#usage)

## Installation
In order to use Qictex, you first need to install LaTex.
To do this, download the installer from [the latex website](https://www.latex-project.org/get/) or directly from [MikTex](https://miktex.org/download).

After installing LaTex, you can install Qictex by
 - Installing rust from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install), cloning the github repository, running "cargo build --release" and moving the ./target/release/qictex.exe file to your preferred installation directory
   or
 - (coming soon!) by directly downloading a precompiled exe file from the repository

## Usage
### With vscode:
 - install vscode
 - DON'T install a LaTex extension because the pdf editor will try to compete with the much better one from tomoki1207.pdf
 - install the tomoki1207.pdf extension
 - create your filename.qictex file
 - open the file with the qictex executable by right-clicking the .qictex file and selecting \"open with\", \"choose another app\", \"other apps\". Finally, scroll down to \"find another app on this pc\" and select this executable in the opened explorer window. This process won't be necessary the second time as windows will automatically remember your choice.
 - this will open a terminal. It will wait for changes in the .qictex file. When it detects one, it automatically compiles it to .tex and starts latex to compile to a pdf. Make sure you have vscode autosave enabled or it won't work!
 - open this pdf in vscode to preview your document.
 - when you're done, just close the terminal.
