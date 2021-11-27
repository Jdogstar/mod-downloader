from concurrent.futures import ThreadPoolExecutor
import requests
import csv
from pathlib import Path
import os
import re
from pyunpack import Archive


def get_mod(mod_details):
    mod_path = Path(mod_details[1])
    mod_http = requests.get(mod_details[0])
    content = mod_http.content
    condis = mod_http.headers['content-disposition']
    filename = get_filename(condis)
    if not filename:
        if not mod_details[2]:
            print("ERROR: no filename from http content, " +
                  "must provide filename in csv")
            return
        else:
            filename = mod_details[2]
    full_path = mod_path / filename
    file_extension = Path(filename).suffix
    with open(full_path, 'wb') as mod_file:
        mod_file.write(content)
    if file_extension in (".ts4script", ".package"):
        return
    elif file_extension in (".zip", ".7z"):
        Archive(full_path).extractall(mod_path)
        os.remove(full_path)
        return
    else:
        print("ERROR: Unaccounted for file extension")
    return


def get_filename(condis):
    if not condis:
        return None
    fname = re.search(r'filename="*([^"]+)"*;', condis)
    if fname:
        return fname.group(1)
    return None


def main():
    mod_list = []
    with open('config.csv') as csvfile:
        mods = csv.reader(csvfile)
        for mod in mods:
            if len(mod) > 2:
                mod_list.append((mod[0].strip(), mod[1].strip(), mod[2].strip()))
            elif len(mod) == 2:
                mod_list.append((mod[0].strip(), mod[1].strip(), None))
            else:
                print("ERROR: missing information for mod entry, please " +
                      "ensure it's in the following form: ")
                print("https://fakeurl.somedownloadlink, " +
                      "C://fullpathtoplacedownload, optional_filename")
    with ThreadPoolExecutor() as process:
        process.map(get_mod, mod_list)


if __name__ == "__main__":
    main()
