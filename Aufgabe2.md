### Aufgabenstellung: Implementierung eines Shape-Traits für verschiedene geometrische Formen

Ziel: Gemeinsame Logik für unterschiedliche Strukturen nutzen. Experimentieren mit generischen Funktionen.

Beschreibung:

1.  Definieren eines Traits:
    1.  Erstellen Sie ein Trait namens Shape mit folgenden Methoden:
        1.  area(&self) -> f64: Gibt die Fläche der Form zurück.
        2.  perimeter(&self) -> f64: Gibt den Umfang der Form zurück.
1.  Erstellen von Strukturen:
    1.  Erstellen Sie eine Struktur Circle mit:
        1.  einem Feld radius: f64, das den Radius des Kreises speichert.
2.  Erstellen Sie eine Struktur Rectangle mit:
    1.  zwei Feldern width: f64 und height: f64, die die Breite und Höhe des Rechtecks speichern.
3.  Implementieren des Traits:
    1.  Implementieren Sie das Shape-Trait für Circle:
        1.  area soll die Fläche des Kreises berechnen: (π×radius)^2.
        2.  perimeter soll den Umfang des Kreises berechnen: 2×π×radius.
    2.  Implementieren Sie das Shape-Trait für Rectangle:
        1.  area soll die Fläche des Rechtecks berechnen: width×height.
        2.  perimeter soll den Umfang des Rechtecks berechnen: 2×(width+height).
4.  Hauptprogramm:
    1.  Erstellen Sie Instanzen eines Circle und eines Rectangle mit Beispielwerten.
    2.  Rufen Sie die Methoden area und perimeter für beide Formen auf und geben Sie die Ergebnisse aus.

Beispielausgabe:

Circle:
- Radius: 5.0
- Area: 78.54
- Perimeter: 31.42

Rectangle:
- Width: 4.0, Height: 7.0
- Area: 28.0
- Perimeter: 22.0