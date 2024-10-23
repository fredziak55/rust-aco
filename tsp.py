import math

# Funkcja do obliczania odległości między dwoma punktami
def odleglosc(punkt1, punkt2):
    x1, y1 = punkt1
    x2, y2 = punkt2
    return math.sqrt((x2 - x1) ** 2 + (y2 - y1) ** 2)

# Funkcja do znalezienia najkrótszej trasy
def najkrotsza_trasa(punkty):
    n = len(punkty)  # liczba punktów
    odwiedzone = [False] * n  # Tablica śledząca odwiedzone punkty
    trasa = []  # Lista do przechowywania kolejności odwiedzonych punktów
    dystans = 0  # Łączny dystans przebytej trasy
    
    
    # Zaczynamy od punktu 0 (pierwszy punkt w liście)
    aktualny_punkt = 0
    trasa.append(aktualny_punkt)
    odwiedzone[aktualny_punkt] = True
    
    # Znajdowanie najkrótszej trasy
    for _ in range(n - 1):  # Odwiedzimy n-1 punktów, bo na końcu wracamy do startu
        najblizszy_punkt = None
        minimalna_odleglosc = float('inf')  # Ustawiamy na bardzo dużą wartość
        
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
punkty = [(1.0, 1.0), (8.0, 5.0), (4.0, 1.0), (1.0, 5.0)]

trasa, dystans = najkrotsza_trasa(punkty)
print(f"Najkrótsza trasa: {trasa}")
print(f"Łączny dystans: {dystans}")