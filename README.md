# Sims Mod Downloader v0.1
## This is just a small script written by me for my gf to automate the download of *most* mods

## Setup install
---
If you are completely new to python and github read this first, otherwise skip to prequisites. First, you need to download this repository as a zip file. Click on the green **CODE** button on the top right and click download as zip. This will download the repository as a zip file. Extract the zip file to where ever you want this program to go.
You then need to install python. There are many tutorials online for installing it, here is an example one https://www.tutorialspoint.com/how-to-install-python-in-windows. Next open up powershell, cmd, or bash, just some sort of command line termainal. Us the command cd to navigate to wherever you stored the unziped file on your computer. Example `cd Desktop/sims4_modder_main/`. Run the command `pip install -r requirements.txt`. Now you can go onto creating the .csv file.

## Prequisites
---
1. You need python verion 3.8+
2. run pip install -r requirements.txt to install needed dependencies using powershell or cmd or bash
3. In the same folder as main.py create a file called config.csv (how to fill out config.csv will be described in a later section)
4. Gather the appropriate information about the mods you want to automate

## Config.csv file syntax
---
This is the config file that the program uses to store what mods you want to download/automate with the program. The syntax for the file is as follows (If you are viewing this file raw, instead of in something that interprets markdown, just ignore the "> " preceding each one of theses lines and the "\" following each line, ***I REPEAT DO NOT PUT THE > or \ IN YOUR CSV FILE***):
> http://exactdownloadurl, C://absolutepathtostoremodto, optional_filename \
> http://exactdownloadurl2, C://absolutepathtostoremodto2, optional_filename2 \
> http://exactdownloadurl3, C://absolutepathtostoremodto3, optional_filename3

- The download url can't be a url to the page with the download link, it must be the download/file link itself. The quick and easy way to find this link is that for most sites, right clicking the download button and clicking **Copy link address** will often give you the download link. Your context clues that you have the right link is that it will have something about either */download* or */file* or something about the site it's downloaded from like */drive* for google drive or */patreon* for patreon.

- The path is just an absolute path to where the mod should go, the easy way to do this is open up any file explorer, navigate to the folder the mod should be placed in, click the bar showing your path and just copy what that gives you. Should look somehting like C, D, E (whatever letter your drive starts with)://yada/something/user/Sims4/mods/wherever/keepgoing/gameplay

- The optional_filename is just there as a backup if my program can't figure out how to name the file from the content request it does. The program will yell **ERROR: no filename from http content, must provide filename in csv** if it can't get the filename from the http content and you haven't provided a backup name in the csv. For the vast majority of files, you should be able to just not add this part of the csv file meaning your entry will look more akin to:
> http://exactdownloadurl, C://absolutepathtostoremodto \
> http://exactdownloadurl2, C://absolutepathtostoremodto2 \
> http://exactdownloadurl3, C://absolutepathtostoremodto3

- Ensure that it is actually a .csv file and not a .txt. Specifically on windows, when you create a new text document, by defualt it will not let you change extensions. Ensure that in your file explorer, you have enabled view file name extensions. If your config filename looks like `config.csv.txt`, just rename and delete the `.txt` and it should work.

## Usage
Assuming you have read and completed the required tasks in the previous section, the usage is pretty simple. Simply open up any given terminal (powershell, command line, bash etc), navigate to where you placed the main.py & config.csv with cd /path/to/folder/holding/files/, and run the following command:
> `python main.py`
Assuming you or ***(VASTLY MORE LIKELY)*** I didn't mess up, it should download all mods you specified in the .csv file.

## Notes and Considerations
- This has only been tested on Patreon, Googledrive, Mediashare, and SimFile share download links. Any other sort of download link is in dangerous territory as I don't even know if the above work 100% of the time. But go for it anyways.

- If you can't get a direct download link, that is a mod you will have to manually download. I have yet to figure out a way to get activate the javascript to hand my progam the download on certain sites like itch.io so that's out of the scope of this script by a long shot.

- When you run the command, it will redownload ALL mods, not just the ones that were updated, if there is great demand for such a system that only downloads updated mods, I will try to figure out a way to do so.

- I slapped this out in a hurry, do not consider this some full fledge program. I will try to patch bugs as they arrive in use by my gf or comments/issues on the github.

- The only *supported* file types for download are .package, .ts4script, .zip, .7z, Everything else will not work. If there are use cases/demand for other extensions I will try to implement them.

- This has only ever been tested on windows 10

- if the folders are not already created in your file system that you want to use in the path, this will not work
