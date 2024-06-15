import subprocess
import os

root = os.getcwd()
commands = [
    ("npm i", "./app/kit"),
    ("npm i", "./app/emulator"),
    ("cargo build", "./")
]

def run_command(command, directory):
    try:
        os.chdir(os.path.join(root, directory))
        print(f"Running '{command}' in {directory}...")
        result = subprocess.run(command, shell=True, check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        print(result.stdout.decode('utf-8'))
    except subprocess.CalledProcessError as e:
        print(f"Error running '{command}' in {directory}: {e.stderr.decode('utf-8')}")

for command, directory in commands:
    run_command(command, directory)