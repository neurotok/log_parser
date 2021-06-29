# log_parser

Parsing console output from Vulkan CTS and generate list of failing tests and report in csv format.

Usage

1. Drop your console output in root direcotry
2. cargo run --release console_output.txt
3. cargo run input_file.txt
4. report.csv and failed.txt files will be generated in the root direcotry
5. You can now re-run CTS only for tests that was faling by running .\deqp-vk --deqp-log-filename=failed.txt 
