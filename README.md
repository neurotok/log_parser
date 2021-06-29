# log_parser

Parse console output from Vulkan CTS and generate list of failing tests and report in csv format.

Usage

1. make sure you have rust installed, if not [go here](https://www.rust-lang.org/tools/install) and proceed
2. drop your console output in root direcotry of this repo
3. cargo run --release console_output.txt
4. report.csv and failed.txt files will be generated in the root direcotry
5. You can now re-run CTS only for tests that was faling by running .\deqp-vk --deqp-log-filename=failed.txt 
