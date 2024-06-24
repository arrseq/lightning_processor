import subprocess
import os

root = os.getcwd()
commands = [
    ("cargo build", "./")
]

for command, directory in commands:
    try:
        os.chdir(os.path.join(root, directory))
        print(f"Running '{command}' in {directory}...")
        result = subprocess.run(command, shell=True, check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        print(result.stdout.decode('utf-8'))
    except subprocess.CalledProcessError as e:
        print(f"Error running '{command}' in {directory}: {e.stderr.decode('utf-8')}")