import os
from subprocess import call

def main():
    # os.chdir('{{ cookiecutter.project_slug }}')
    # call(['git', 'init'])
    # call(['git', 'add', '.'])
    # call(['git', 'commit', '-m', 'Initial commit'])
    print(f'Project initialized. Run `cd {{ cookiecutter.project_slug }}` to enter the project directory.')

if __name__ == '__main__':
    main()
