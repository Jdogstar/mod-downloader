# Mod Downloader v0.1
## This is just a small program written by me for my gf to automate the download of *most* mods for the sims4 and other games

## Setup & install
---
Simply head to the latest [release](https://github.com/Jdogstar/mod-downloader/releases) on the github page and grab the executable for your os. If you don't see your os, open an issue on github and let me add it. Be sure to add executable permissions to the executable after downloading it. Feel free to rename the executable to something shorter or more personalized as well.

## prerequisites
---
1. You need 7zip installed
2. In the same folder as the executable create a file .csv file named whatever you want (details on how to fill out this file are described later on)
3. Gather the appropriate information about the mods you want to automate

## Config.csv file syntax
---
This is the config file that the program uses to store what mods you want to download/automate with the program. The syntax for the file is as follows (If you are viewing this file raw a.k.a not on github, just ignore the "> " preceding each one of theses lines and the "\" following each line, DO NOT PUT THE **>** or **\\** IN YOUR CSV FILE):
> C://absolutepathtostoremodto, http://exactdownloadurl, optional_filename \
> C://absolutepathtostoremodto2, http://exactdownloadurl2, optional_filename2 \
> C://absolutepathtostoremodto3, http://exactdownloadurl3, optional_filename3

- The download url can't be a url to the page with the download link, it must be the download/file link itself. The quick and easy way to find this link is that for most sites, right clicking the download button and clicking **Copy link address** will often give you the download link. Your context clues that you have the right link is that it will have something about either */download* or */file* or something about the site it's downloaded from like */drive* for google drive or */patreon* for patreon.

- The path is just an absolute path to where the mod should go, the easy way to do this is open up any file explorer, navigate to the folder the mod should be placed in, click the bar showing your path and just copy what that gives you. Should look somehting like C, D, E (whatever letter your drive starts with)://yada/something/user/Sims4/mods/wherever/keepgoing/gameplay

- The optional_filename is just there as a backup if my program can't figure out how to name the file from the content request it does. The program will yell **ERROR: no filename from http content, must provide filename in csv** if it can't get the filename from the http content and you haven't provided a backup name in the csv. For the vast majority of files, you should be able to just not add this part of the csv file meaning your entry will look more akin to:
> C://absolutepathtostoremodto, http://exactdownloadurl \
> C://absolutepathtostoremodto2, http://exactdownloadurl2 \
> C://absolutepathtostoremodto3, http://exactdownloadurl3

- Ensure that it is actually a .csv file and not a .txt. Specifically on windows, when you create a new text document, by default it will not let you change extensions. Ensure that in your file explorer, you have enabled view file name extensions. If your config filename looks like `config.csv.txt`, just rename and delete the `.txt` and it should work.

## Usage
Assuming you have read and completed the required tasks in the previous section, the usage is pretty simple. Simply open up any given terminal (powershell, command line, bash etc), navigate to where you placed the executable & csv file with cd /path/to/folder/holding/files/, and run the following command:
> `executable_name --path csv_name.csv` \
> `mod_downloader_linux --path test.csv`

Assuming you or ***(MORE LIKELY)*** I didn't mess up, it should download all mods you specified in the .csv file.

## Logs
In the folder the executable is located in, it will generate a folder called logs. Inside will be logs of every time you've run the application. Feel free to delete these from time to time. If something isn't working correctly, either the program is crashing or a mod is failing to download, then you can look at the logs to see what's wrong and upload them when you report the issue.

## Notes and Considerations
- This has only been tested on Patreon, Googledrive, and SimFile share download links. Any other sort of download link is in dangerous territory as I don't even know if the above work 100% of the time. But go for it anyways.

- If you can't get a direct download link, that is a mod you will have to manually download. I have yet to figure out a way to get activate the javascript to hand my program the download on certain sites like itch.io so that's out of the scope of this script by a long shot.

- When you run the command, it will redownload ALL mods, not just the ones that were updated, if there is great demand for such a system that only downloads updated mods, I will try to figure out an efficient way to do so.

- I slapped this out in a hurry, do not consider this some full fledge program. I will try to patch bugs as they arrive in use by my gf or comments/issues on the github. I will also slowly try to add more features, especially if there is demand.

- The only *supported* file types for download are .package, .ts4script, .zip, .7z, and other various common zip types. Everything else will not work. If there are use cases/demand for other extensions I will try to implement them.

- This has only ever been tested on windows 10 and the latest arch linux

- if the folders are not already created in your file system that you want to use in the absolute path in the csv the program will not download that mod successfully.

## Potential TODO list
These are features I would like to slowly integrate as I iteratively turn this into a more full fledged program
- [x] Convert to rust
- [ ] Automatically build paths if they do not exist
- [ ] Detect if mods have or have not been updated using http headers
- [ ] Instructions on how to put the binaries in the path, allowing for use anywhere
- [ ] Remove the need for the csv file, replace with config files created by the program to facilitate path usage
- [ ] Flesh out command line features and parameters to include ability for the following
    - [ ] Add a mod to the config
    - [ ] List current mods, and details
    - [ ] Remove a mod by name, deleting the mod
    - [ ] Move where a mod downloads to, which will also move the mod if it exists
- [ ] Add automatic deletion of logs if no fatal error is detected
- [ ] Move logs to set location
- [ ] Improve logs for most actions with more details
- [ ] Refactor most parts of the program
- [ ] Investigate using selenium to add custom support for some websites aren't currently downloadable from