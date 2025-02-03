## **Aufgabenstellung: Benutzerinteraktion mit geometrischen Formen**

**Ziel** 
Dynamisches Arbeiten mit Structs und Traits anhand der Benutzereingaben.

**Beschreibung:**
Erweitern Sie das vorhandene Programm, um Benutzerinteraktionen hinzuzufügen. Der Benutzer soll die Möglichkeit haben, zur Laufzeit eine geometrische Form auszuwählen, die er erstellen möchte, und die notwendigen Parameter (z. B. Radius, Breite, Höhe) einzugeben. Basierend auf der Auswahl und den Eingaben des Benutzers berechnet das Programm die Fläche und den Umfang der gewählten Form und gibt diese aus.

### **Schritte zur Umsetzung:**

1.  **Menü anzeigen:**

    -   Zeigen Sie ein Menü mit Optionen an, damit der Benutzer zwischen folgenden Möglichkeiten wählen kann:

        -   **1\. Kreis erstellen (Circle).**

        -   **2\. Rechteck erstellen (Rectangle).**

        -   **3\. Programm beenden.**

2.  **Eingabe der Parameter:**

    -   Für die Erstellung eines Kreises soll der Benutzer den **Radius** eingeben.

    -   Für die Erstellung eines Rechtecks soll der Benutzer die **Breite** und **Höhe** eingeben.

3.  **Berechnung und Ausgabe:**

    -   Nachdem der Benutzer die Parameter eingegeben hat, berechnet das Programm die **Fläche** und den **Umfang** der gewählten Form.

Beispielausgabe: Sie haben einen Kreis mit Radius 5.0 gewählt.

Fläche: 78.54

Umfang: 31.42

1.  **Fehlerbehandlung:**

    -   Wenn der Benutzer ungültige Eingaben macht (z. B. negative Werte oder nicht-numerische Eingaben), soll das Programm den Fehler anzeigen und die Eingabe erneut anfordern.

2.  **Programm wiederholen:**

    -   Nach der Berechnung kehrt das Programm zum Hauptmenü zurück, damit der Benutzer eine neue Form erstellen oder das Programm beenden kann.

**Damit das Programm Benutzereingaben verarbeiten kann, werden folgende Module, Methoden und Funktionen benötigt:**

### **📍** Benötigtes Modul:

-   `std::io` → Wird für das Einlesen von Benutzereingaben genutzt.

### 📍 Wichtige Methoden & Funktionen:

-   `read_line(&mut String)` → Liest eine Eingabe aus der Konsole und speichert sie als String.

-   `trim()` → Entfernt Leerzeichen und Zeilenumbrüche aus der Eingabe.

-   `parse::<u32>()` / `parse::<f64>()` → Konvertiert eine Zeichenkette in eine Zahl (u32 für Menüoptionen, f64 für numerische Eingaben).

-   `expect("Fehlermeldung")` → Gibt eine Fehlermeldung aus, wenn die Eingabe nicht verarbeitet werden kann.

-   `unwrap_or(-1.0)` → Falls die Konvertierung fehlschlägt, wird ein Standardwert (hier -1.0) zurückgegeben.

-   Eigene Hilfsfunktion `read_input()`

    -   Liest eine f64-Zahl aus der Konsole.

    -   Behandelt fehlerhafte Eingaben, indem sie -1.0 zurückgibt.

### 📍 Ablauf für die Benutzereingabe:

1.  Der Benutzer trifft eine Auswahl (Kreis, Rechteck oder Beenden).

2.  Die Eingabe wird mit `read_line()` gelesen und mit `trim()` und `parse::<u32>()` in eine Zahl umgewandelt.

3.  Mit match wird geprüft, welche Option gewählt wurde.

4.  Falls eine Form gewählt wurde, werden die notwendigen Werte (radius, width, height) mit der Hilfsfunktion `read_input()` eingelesen.

5.  Falls falsche Werte eingegeben wurden (negative Zahlen oder ungültige Eingaben), wird eine Fehlermeldung ausgegeben und eine erneute Eingabe angefordert.

### Um die Benutzerwahl zu verarbeiten und das Programm wiederholt Eingaben abfragen zu lassen, werden folgende Kontrollstrukturen und Schleifen genutzt:

### 📍 Wichtige Kontrollstrukturen:

-   `match choice {}`

    -   Prüft, ob der Benutzer eine gültige Option gewählt hat (1 für Kreis, 2 für Rechteck, 3 für Beenden).

    -   Falls die Eingabe ungültig ist, wird eine Fehlermeldung ausgegeben.

-   `if-Bedingungen`

    -   Überprüft, ob die eingegebenen Werte (z. B. radius, width, height) größer als 0.0 sind.

    -   Falls nicht, gibt das Programm eine Fehlermeldung aus und fordert eine erneute Eingabe.

### 📍 Wichtige Schleife & Steuerbefehle:

-   `loop {}`

    -   Erstellt eine Endlosschleife, damit das Menü nach jeder Berechnung erneut angezeigt wird.

    -   Die Schleife läuft, bis der Benutzer sich für "Beenden" entscheidet.

-   `break`

    -   Beendet die Schleife, wenn der Benutzer 3 wählt.

-   `continue`

    -   Falls eine fehlerhafte Eingabe erkannt wird (z. B. radius <= 0.0), wird der aktuelle Durchlauf übersprungen und das Menü erneut angezeigt.

### **Beispielablauf:**

Wählen Sie eine Form:

1\. Circle

2\. Rectangle

3\. Exit

Ihre Wahl: 1

Geben Sie den Radius des Kreises ein:

5.0

Sie haben einen Kreis mit Radius 5.0 gewählt.

Fläche: 78.54

Umfang: 31.42

Wählen Sie eine Form:

1\. Circle

2\. Rectangle

3\. Exit

Ihre Wahl: 2

Geben Sie die Breite des Rechtecks ein:

4.0

Geben Sie die Höhe des Rechtecks ein:

7.0

Sie haben ein Rechteck mit Breite 4.0 und Höhe 7.0 gewählt.

Fläche: 28.00

Umfang: 22.00

Wählen Sie eine Form:

1\. Circle

2\. Rectangle

3\. Exit

Ihre Wahl: 3

Programm beendet.