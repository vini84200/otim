import numpy as np
import pandas as pd
import glob
import re
import locale
locale.setlocale(locale.LC_ALL, '')

rx_dict = {
    'inputfile': re.compile(r'Input file: (?P<inputfile>.*)\n'),
    'seed': re.compile(r'RNG seed: (?P<seed>.*)\n'),
	'iter_resfriamento': re.compile(r'Iteracoes para resfriamento: (?P<iter_resfriamento>.*)\n'),
	'iter_metropoles': re.compile(r'Iteracoes para metropoles: (?P<iter_metropoles>.*)\n'),
	'resfriamento': re.compile(r'Resfriamento: (?P<resfriamento>.*)\n'),
	'temperatura': re.compile(r'Temperatura inicial: (?P<temperatura>.*)\n'),
	'tempo_real': re.compile(r'Tempo real (?P<tempo_real>.*)\n'),
	'tempo_cpu': re.compile(r'Tempo CPU (?P<tempo_cpu>.*)\n'),
	'value': re.compile(r'Final one: (?P<value>.[0-9]+)\n'),
}

def parse_line(line):
    for key, rx in rx_dict.items():
        match = rx.search(line)
        if match:
            return key, match
    # if there are no matches
    return None, None

def to_seconds(val:str):
	if val.endswith('ms'):
		return locale.atof(val.replace('ms','').replace('.',','))/1000
	else:
		return locale.atof(val.replace('s','').replace('.',','))


data = pd.DataFrame()

paths = list(map(lambda p: (int(p.split('\\')[1].split('.')[0]), p), glob.glob("./logs/*.log")))
sorted_paths = sorted(paths, key=lambda t: t[0])
for i, filepath in sorted_paths:
	file_data = {}
	with open(filepath, 'r') as file_obj:
		for line in file_obj.readlines():
			key, match  = parse_line(line) 
			match key:
				case None:
					continue
				case 'inputfile':
					file_data[key] = match.group(key).replace('\"','')
				case 'tempo_real':
					file_data[key] = f'{to_seconds(match.group(key)):.3f}s'
				case 'tempo_cpu':
					file_data[key] = f'{to_seconds(match.group(key)):.3f}s'
				case _:
					file_data[key] = match.group(key)
	f_data = pd.DataFrame([file_data])
	file_data.clear()
	data = pd.concat([data, f_data], ignore_index=True)
#data = data.append([file_data], ignore_index=True, sort=False)
#print(data)
	
data.to_csv('all_logs.csv')


