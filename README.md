# Neutron

Neutron est un langage de programmation expérimental conçu pour explorer la création de langages personnalisés, avec une syntaxe simple et un pipeline complet (lexer, parser, analyse sémantique).

## 📚 Description

L'objectif de **Neutron** est de fournir un environnement pour comprendre et implémenter les concepts fondamentaux des langages de programmation, notamment :

- **Lexing** : Analyse lexicale pour convertir le code source en une séquence de jetons.
- **Parsing** : Analyse syntaxique pour construire une représentation hiérarchique (AST).
- **Analyse sémantique** : Vérification des règles sémantiques (déclarations, portée, etc.).
- **Exécution** (futur) : Implémentation d'un interpréteur ou compilateur.

Exemple de code en Neutron :
```neutron
class MyClass then
    var x
    func myFunc then
        x = 42
    end
end
