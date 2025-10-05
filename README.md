# Resume and Portfolio Site Builder
This project serves the dual purpose of maintaining a resume and personal portfolio website with one source of truth expressed from JSON documents.

The Rust program will take the data from the public folder and place it in the NextApp for use in the website. Users should only have to update the JSON documents in the public folder within the root/input/ directory to update both the resume and website. New pages needing custom configuration should be done within the NextApp as well and made to use the JSON documents.

If you would like to use the document as well, rather than posting complex instructions, feel free to contact me and I can walk you through how to use it. If you don't need me to walk you through it, you can use it as a template for your personal portfolio as well.

## External Dependencies
Currently relies on ***pdflatex*** command being installed on the system for compiling LaTeX into PDF.

## Future Plans
* More modular approach to building resume LaTeX and PDF files. This would allow others to use the software more freely for their own resume/portfolio generation
* Implement Rust tests to ensure valid data is being used for both resume and portfolio
* Make code easier to read and maintain for future development