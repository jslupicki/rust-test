import subprocess
import re
import os
import shutil

try:
    out = subprocess.check_output(
        ["cargo", "test", "--", "--non-existent-par"], stderr=subprocess.STDOUT)
except subprocess.CalledProcessError as e:
    out = e.output

out_text = out.decode("UTF-8")
print("****************")
print(out_text)
print("****************")

m = re.search("Running (.+)\n", out_text)
if m:
    file_txt = m.group(1)
    print("Find:", file_txt)
else:
    print("Not found!")
    exit

sep_idx = file_txt.rindex(os.sep) + 1

path = file_txt[:sep_idx]
file_name = file_txt[sep_idx:]
ext = file_name[file_name.rindex("."):]
dest = path + "tests_for_debug" + ext

print("path:", path)
print("file_name:", file_name)
print("ext", ext)
print("dest", dest)

shutil.copy(file_txt, dest)
