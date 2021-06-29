# log_parser

Parsing console output from Vulkan CTS and generate list of failing tests and report csv format.

Usage

1. Drop your console output in root direcotry
2. cargo build --release
3. cargo run input_file.txt
4. Wait, this may take a while
5. report.csv and failed.txt files will be generated in the root direcotry
6. YOu can now re-run CTS only for files that was faling running .\deqp-vk --deqp-log-filename=failed.txt 
