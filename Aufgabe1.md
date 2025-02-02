### Aufgabenstellung

1\. Einführung

Ziel: In Rust Traits definieren, implementieren und mit Structs kombinieren

1.  Definieren eines Traits:
    1.  Erstellen Sie ein Trait namens Animal, das die Methode greet(&self) -> String enthält.
    2.  Diese Methode soll eine Begrüßung des Tiers als String zurückgeben (z. B. ein typisches Geräusch, das das Tier macht).
2.  Erstellen von Structs:
    1.  Erstellen Sie eine Struktur  Cat mit einem Feld name: String, das den Namen der Katze speichert.
    2.  Erstellen Sie eine Struktur Dog mit einem Feld name: String, das den Namen des Hundes speichert.
3.  Implementieren des Traits:
    1.  Implementieren Sie das Animal-Trait für Dog. Die Methode greet  soll den Text  "rof rof rof" zurückgeben.   2.  Implementieren Sie das Animal-Trait für Cat. Die Methode greet kann z. B. "meow meow" zurückgeben.
4.  Hauptprogramm:
    1.  Erstellen Sie im  main eine Instanz von Dog mit einem Beispielnamen (z. B. "Max").
    2.  Geben Sie den Namen des Hundes und die Begrüßung aus, indem Sie die Methode  greet aufrufen.
    3.  Bonus (optional): Erstellen Sie auch eine Instanz von Cat und geben Sie die Begrüßung aus.

Beispielausgabe:

Dog's name: Max

Dog says: rof rof rof

Cat's name: Luna

Cat says: meow meow