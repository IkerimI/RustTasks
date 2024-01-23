# Übungsblatt 4
## Allgemeine Hinweise
Für diese und alle folgenden Praktikumsaufgaben gilt, dass Einsendungen, die in der jeweils mitgegebenen Testumgebung nicht laufen, Punktabzug erhalten. 
Das beinhaltet insbesondere alle Programme, die sich nicht fehlerfrei kompilieren lassen.
Da Cargo für die Ausführung verantwortlich ist, sollte das Projekt bei Ihnen am Ende mit `cargo test` ohne Fehler und Warnungen durchlaufen.


## Abgabemodus
Die Lösung ist in einem eigenen Git-Repository abzugeben.
Sie können in ihrer Lösung beliebige Hilfstypen und Module selbst definieren, jedoch dürfen die vorhandenen Testfälle nicht abgeändert werden.

Zur Lösung der Aufgaben steht für Sie dieses Repository mit
- einer Testdatei [demorgan](src/parser/demorgan.c1),
- einem Lexer in [minako-lexic](src/parser/lexer.rs)
- einem Parser-Template in [minako-syntax](src/parser/minako_syntax.y)

zur Verfügung.

## Aufgabenstellung (100 Punkte)
Der von uns verwendete Parsergenerator *bison* sollte Ihnen bereits aus der Übung bekannt sein. Da Materialien zu Bison (Handbücher, Tutorials, …) sehr zahlreich im Netz vorhanden sind, wird hier auf eine weitere Erklärung verzichtet.

Für Rust bietet die [rust-bison-skeleton](https://crates.io/crates/rust-bison-skeleton) crate ein Bison frontend an, welches Parser in Rust generiert. Nähere Informationen finden Sie in der README der crate, und in der Vorbesprechung des Übungsblatts.
Bitte beachten Sie, dass die crate lediglich ein anderes Frontend für bison bereitstellt. __Bison muss auf ihrem Rechner installiert sein, um die crate nutzen zu können__. Hierfür wird sehr wahrscheinlich Linux benötigt.
Unter Windows bietete sich die Nutzung von [WSL](https://learn.microsoft.com/en-us/windows/wsl/install) an. Für CLion und VSCode stehen entsprechende WSL-Integrationen zur Verfügung. 

Ihre Aufgabe besteht darin, einen Parser für die Sprache C1 in Bison zu spezifizieren. Dazu bekommen Sie von uns einen Lexer. (Sie können theoretisch auch Ihren Scanner vom Aufgabenblatt 2 benutzen – aber dessen Integration in den Parser erfordert etwas Aufwand)

Die Grammatik von C1 finden Sie [hier](https://amor.cms.hu-berlin.de/~kunert/lehre/material/c1-grammar.php) sowie nachfolgend:

```c
program             ::= ( declassignment ";" | functiondefinition )*

functiondefinition  ::= type id "(" ( parameterlist )? ")" "{" statementlist "}"
parameterlist       ::= type id ( "," type id )*
functioncall        ::= id "(" ( assignment ( "," assignment )* )? ")"

statementlist       ::= ( block )*
block               ::= "{" statementlist "}"
                      | statement

statement           ::= ifstatement
                      | forstatement
                      | whilestatement
                      | returnstatement ";"
                      | dowhilestatement ";"
                      | printf ";"
                      | declassignment ";"
                      | statassignment ";"
                      | functioncall ";"
statblock           ::= "{" statementlist "}"
                      | statement

ifstatement         ::= <KW_IF> "(" assignment ")" statblock ( <KW_ELSE> statblock )?
forstatement        ::= <KW_FOR> "(" ( statassignment | declassignment ) ";" expr ";" statassignment ")" statblock
dowhilestatement    ::= <KW_DO> statblock <KW_WHILE> "(" assignment ")"
whilestatement      ::= <KW_WHILE> "(" assignment ")" statblock
returnstatement     ::= <KW_RETURN> ( assignment )?
printf              ::= <KW_PRINTF> "(" (assignment | CONST_STRING) ")"
declassignment      ::= type id ( "=" assignment )?

statassignment      ::= id "=" assignment
assignment          ::= id "=" assignment
                      | expr
expr                ::= simpexpr ( "==" simpexpr | "!=" simpexpr | "<=" simpexpr | ">=" simpexpr | "<" simpexpr | ">" simpexpr )?
simpexpr            ::= ( "-" term | term ) ( "+" term | "-" term | "||" term )*
term                ::= factor ( "*" factor | "/" factor | "&&" factor )*
factor              ::= <CONST_INT>
                      | <CONST_FLOAT>
                      | <CONST_BOOLEAN>
                      | functioncall
                      | id
                      | "(" assignment ")"


type                ::= <KW_BOOLEAN>
                      | <KW_FLOAT>
                      | <KW_INT>
                      | <KW_VOID>
id                  ::= <ID>
```

Da Bison leider keine EBNF, sondern nur BNF versteht, werden Sie die Grammatik zwangsläufig umbauen müssen.
Dabei ist im Prinzip fast alles erlaubt, nur die Sprache darf sich natürlich nicht verändern!

### Folgende Anforderungen werden an Ihre Lösung gestellt:

- die Implementation erfolgt in der Datei minako-syntax.y
- der Parser gibt im erfolgreichen Fall nichts aus und das Programm endet mit Rückgabewert '0'
- bei einem Parserfehler wird eine Fehlermeldung ausgegeben und das Programm mit der Rückgabe des Fehlers beendet
- wenn Sie den Parser mithilfe der Tests auf das mitgelieferte C1-Programm demorgan.c1 ansetzen, sollte er entsprechend nichts ausgeben und alle Tests erfolgreich durchlaufen
- die Verwendung von %expect x für x > 0 im Bison-Code ist nicht gestattet

### Zur Hilfestellung seien noch folgende Hinweise gegeben:
- Gehen Sie am Anfang alle EBNF-Konstrukte durch und überlegen Sie sich, wie man diese jeweils generisch in BNF umwandeln kann.
- In dieser Grammatik ist eine Mehrdeutigkeit enthalten, die einem spätestens bei der Implementierung auffällt.
- Rufen Sie sich die Bedeutung von %left, %right und %nonassoc in Erinnerung, bevor Sie die Grammatik unnötig verkomplizieren, da das Template Mehrdeutigkeiten hiermit bereits auflöst. 
