import time

def wczytaj_punkty_z_pliku(file_path):
    with open(file_path, 'r') as file:
        lines = file.readlines()
    
    num_points = int(lines[0].strip())
    points = []
    
    for line in lines[1:num_points + 1]:
        parts = line.split()
        x = float(parts[1])
        y = float(parts[2])
        points.append((x, y))
    
    return points

def odleglosc(p1, p2):
    return ((p1[0] - p2[0]) ** 2 + (p1[1] - p2[1]) ** 2) ** 0.5

def najkrotsza_trasa(punkty):
    n = len(punkty)
    odwiedzone = [False] * n
    trasa = []
    dystans = 0.0

    aktualny_punkt = 0
    trasa.append(aktualny_punkt)
    odwiedzone[aktualny_punkt] = True

    for _ in range(n - 1):
        minimalna_odleglosc = float('inf')
        najblizszy_punkt = None

        # Przeszukiwanie nieodwiedzonych punktów
        for i in range(n):
            if not odwiedzone[i]:  # Jeśli punkt nie został odwiedzony
                odleglosc_do_i = odleglosc(punkty[aktualny_punkt], punkty[i])
                if odleglosc_do_i < minimalna_odleglosc:
                    minimalna_odleglosc = odleglosc_do_i
                    najblizszy_punkt = i
        
        # Przechodzimy do najbliższego punktu
        aktualny_punkt = najblizszy_punkt
        trasa.append(aktualny_punkt)
        odwiedzone[aktualny_punkt] = True
        dystans += minimalna_odleglosc
    
    # Powrót do punktu startowego (punktu 0)
    dystans = dystans + odleglosc(punkty[aktualny_punkt], punkty[0])
    trasa.append(0)  # Wracamy do punktu startowego
    
    return trasa, dystans

# Przykład działania programu
start_time = time.time()

punkty = wczytaj_punkty_z_pliku('dane.txt')
trasa, dystans = najkrotsza_trasa(punkty)

end_time = time.time()
execution_time = end_time - start_time

# print(f"Najkrótsza trasa: {trasa}")
print(f"Łączny dystans: {dystans}")
print(f"Czas wykonania: {execution_time} sekund")