#!/bin/bash

# The file to split
input_file="microvoid.gtr"
# The size of each split in bytes (16KB)
split_size=$((16*1024))
# The starting bank in hexadecimal
start_bank=0x7E

# Calculate the number of splits needed (assuming bash 5.0+ for floating point support in division)
file_size=$(stat -c%s "$input_file")
num_splits=$((file_size / split_size))
if (( file_size % split_size != 0 )); then
  ((num_splits++))
fi

# Split the file
for ((i=0; i<num_splits; i++)); do
  # Calculate the hex bank value
  printf -v bank_hex "%02X" $((start_bank + i))
  # Calculate the skip amount
  skip_bytes=$((split_size * i))
  # Generate the output filename
  output_file="${input_file}.bank${bank_hex}"
  # Use dd to create the split file
  dd if="$input_file" of="$output_file" bs=$split_size count=1 skip=$i 2>/dev/null
  echo "Created $output_file"
done

echo "Splitting complete."
