from concurrent.futures import ThreadPoolExecutor
import requests
import csv
from pathlib import Path
import os
import re
from pyunpack import Archive


def get_mod(mod_details):
    """Function to run in thread to handle each mod.

    Threading function to download the mod using requests.
    It then takes certain actions based on the file extension.

    Args:
        mod_details: a tuple holding the url, absolute path to save to,
        and the optional filename. Filename is None
        if it's not provided in the tuple.
    """
    # get the absolute path
    mod_path = Path(mod_details[1])
    # grab the file from the url
    mod_http = requests.get(mod_details[0])
    # seperate into content header and the actual file bytes
    content = mod_http.content
    condis = mod_http.headers['content-disposition']
    # get the filename
    filename = get_filename(condis)
    # if there is no filename from the content header
    if not filename:
        # if there is no optional name either, print error, skip mod
        if not mod_details[2]:
            print("ERROR: no filename from http content, " +
                  "must provide filename in csv")
            return
        else:
            # else use optional name provided in the .csv
            filename = mod_details[2]
    # full path to save mod to
    full_path = mod_path / filename
    # get filename extension
    file_extension = Path(filename).suffix
    # write the mod file to the save path
    with open(full_path, 'wb') as mod_file:
        mod_file.write(content)
    # if it's a simple script or package, just return, job done
    if file_extension in (".ts4script", ".package"):
        return
    # if it's a zip or 7z, use the archive module to extract files in the zip
    elif file_extension in (".zip", ".7z"):
        Archive(full_path).extractall(mod_path)
        # remove the original zip as it's no longer needed
        os.remove(full_path)
        return
    # report unaccounted for file extension, but keep the download
    else:
        print("ERROR: Unaccounted for file extension")
    return


def get_filename(condis):
    """Gets filename from content-disposition header.

    Args:
        condis: Uses content-disposition header from requests module.
    Returns:
        None: if it either doesn't recieve a content-disposition header or
        can't find the filename.
        fname: filename from filename content header.
    """
    # if nothing provided, return none
    if not condis:
        return None
    # seach for the filename tag in the header
    fname = re.search(r'filename="*([^"]+)"*;', condis)
    # if there is a match, return the match, otherwise return none
    if fname:
        return fname.group(1)
    return None


def main():
    """Main driver for threads to download mods."""
    mod_list = []
    # grab config file in same folder as main.py
    with open('config.csv') as csvfile:
        mods = csv.reader(csvfile)
        # for row in the csv file
        for mod in mods:
            # if it has all three components, tuple them and add to list
            if len(mod) > 2:
                mod_list.append((mod[0].strip(), mod[1].strip(), mod[2].strip()))
            # else if it has two components, tuple them with the filename as none
            elif len(mod) == 2:
                mod_list.append((mod[0].strip(), mod[1].strip(), None))
            # else complain about the format for that particular row, but continue regardless
            else:
                print("ERROR: missing information for mod entry, please " +
                      "ensure it's in the following form: ")
                print("https://fakeurl.somedownloadlink, " +
                      "C://fullpathtoplacedownload, optional_filename")
    with ThreadPoolExecutor() as process:
        # map each row to a thread from the threadpool
        process.map(get_mod, mod_list)


if __name__ == "__main__":
    main()
