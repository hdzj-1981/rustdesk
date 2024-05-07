lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Stav"),
        ("Your Desktop", "Vaša plocha"),
        ("desk_tip", "K svojej ploche sa môžete pripojiť pomocou zobrazeného ID a hesla."),
        ("Password", "Heslo"),
        ("Ready", "Pripravené"),
        ("Established", "Nadviazané"),
        ("connecting_status", "Pripájam sa na RustDesk server..."),
        ("Enable service", "Povoliť službu"),
        ("Start service", "Spustiť službu"),
        ("Service is running", "Služba je aktívna"),
        ("Service is not running", "Služba je vypnutá"),
        ("not_ready_status", "Nepripravené. Skontrolujte svoje sieťové pripojenie."),
        ("Control Remote Desktop", "Ovládať vzdialenú plochu"),
        ("Transfer file", "Prenos súborov"),
        ("Connect", "Pripojiť"),
        ("Recent sessions", "Nedávne pripojenie"),
        ("Address book", "Adresár kontaktov"),
        ("Confirmation", "Potvrdenie"),
        ("TCP tunneling", "TCP tunelovanie"),
        ("Remove", "Odstrániť"),
        ("Refresh random password", "Aktualizovať náhodné heslo"),
        ("Set your own password", "Nastavte si svoje vlastné heslo"),
        ("Enable keyboard/mouse", "Povoliť klávesnicu/myš"),
        ("Enable clipboard", "Povoliť schránku"),
        ("Enable file transfer", "Povoliť prenos súborov"),
        ("Enable TCP tunneling", "Povoliť TCP tunelovanie"),
        ("IP Whitelisting", "Zoznam povolených IP adries"),
        ("ID/Relay Server", "ID/Prepojovací server"),
        ("Import server config", "Importovať konfiguráciu servera"),
        ("Export Server Config", "Exportovať konfiguráciu servera"),
        ("Import server configuration successfully", "Konfigurácia servera bola úspešne importovaná"),
        ("Export server configuration successfully", "Konfigurácia servera bola úspešne exportovaná"),
        ("Invalid server configuration", "Neplatná konfigurácia servera"),
        ("Clipboard is empty", "Schránka je prázdna"),
        ("Stop service", "Zastaviť službu"),
        ("Change ID", "Zmeniť ID"),
        ("Your new ID", "Vaše nové ID"),
        ("length %min% to %max%", "dĺžka medzi %min% a %max%"),
        ("starts with a letter", "začína písmenom"),
        ("allowed characters", "povolené znaky"),
        ("id_change_tip", "Povolené sú len znaky a-z, A-Z, 0-9 a _ (podčiarkovník). Prvý znak musí byť a-z, A-Z. Dĺžka musí byť medzi 6 a 16 znakmi."),
        ("Website", "Webová stránka"),
        ("About", "O RustDesk"),
        ("Slogan_tip", "Stvorené srdcom v tomto chaotickom svete!"),
        ("Privacy Statement", "Vyhlásenie o ochrane osobných údajov"),
        ("Mute", "Stíšiť"),
        ("Build Date", "Dátum zostavenia"),
        ("Version", "Verzia"),
        ("Home", "Domov"),
        ("Audio Input", "Zvukový vstup"),
        ("Enhancements", "Vylepšenia"),
        ("Hardware Codec", "Hardvérový kodek"),
        ("Adaptive bitrate", "Adaptívny dátový tok"),
        ("ID Server", "ID server"),
        ("Relay Server", "Prepojovací server"),
        ("API Server", "API server"),
        ("invalid_http", "Musí začínať http:// alebo https://"),
        ("Invalid IP", "Neplatná IP adresa"),
        ("Invalid format", "Neplatný formát"),
        ("server_not_support", "Zatiaľ serverom nepodporované"),
        ("Not available", "Nie je k dispozícii"),
        ("Too frequent", "Príliš často"),
        ("Cancel", "Zrušiť"),
        ("Skip", "Preskočiť"),
        ("Close", "Zatvoriť"),
        ("Retry", "Zopakovať"),
        ("OK", "OK"),
        ("Password Required", "Vyžaduje sa heslo"),
        ("Please enter your password", "Zadajte vaše heslo"),
        ("Remember password", "Zapamätať heslo"),
        ("Wrong Password", "Chybné heslo"),
        ("Do you want to enter again?", "Chcete ho znova zadať?"),
        ("Connection Error", "Chyba spojenia"),
        ("Error", "Chyba"),
        ("Reset by the peer", "Odmietnuté druhou stranou spojenia"),
        ("Connecting...", "Pripájanie sa..."),
        ("Connection in progress. Please wait.", "Pokúšam sa pripojiť. Počkajte chvíľu."),
        ("Please try 1 minute later", "Skúte znova za minútu, alebo ešte neskôr"),
        ("Login Error", "Chyba prihlásenia"),
        ("Successful", "Úspech"),
        ("Connected, waiting for image...", "Pripojené, čakám na obraz..."),
        ("Name", "Názov"),
        ("Type", "Typ"),
        ("Modified", "Zmenené"),
        ("Size", "Veľkosť"),
        ("Show Hidden Files", "Zobraziť skryté súbory"),
        ("Receive", "Prijať"),
        ("Send", "Odoslať"),
        ("Refresh File", "Aktualizovať súbor"),
        ("Local", "Miestne"),
        ("Remote", "Vzdialené"),
        ("Remote Computer", "Vzdialený počítač"),
        ("Local Computer", "Miestny počítač"),
        ("Confirm Delete", "Potvrdenie zmazania"),
        ("Delete", "Zmazať"),
        ("Properties", "Vlastnosti"),
        ("Multi Select", "Viacnásobný výber"),
        ("Select All", "Vybrať všetko"),
        ("Unselect All", "Zrušiť výber všetkého"),
        ("Empty Directory", "Prázdny adresár"),
        ("Not an empty directory", "Nie prázdny adresár"),
        ("Are you sure you want to delete this file?", "Ste si istý, že chcete zmazať tento súbor?"),
        ("Are you sure you want to delete this empty directory?", "Ste si istý, že chcete zmazať tento adresár?"),
        ("Are you sure you want to delete the file of this directory?", "Ste si istý, že chcete zmazať tento súbor alebo adresár?"),
        ("Do this for all conflicts", "Všetky konflikty riešiť týmto spôsobom"),
        ("This is irreversible!", "Toto je nezvratná operácia!"),
        ("Deleting", "Mazanie"),
        ("files", "súbory"),
        ("Waiting", "Čaká sa"),
        ("Finished", "Ukončené"),
        ("Speed", "Rýchlosť"),
        ("Custom Image Quality", "Vlastná kvalita obrazu"),
        ("Privacy mode", "Režim súkromia"),
        ("Block user input", "Blokovať vstupné zariadenia užívateľa"),
        ("Unblock user input", "Odblokovať vstupné zariadenia užívateľa"),
        ("Adjust Window", "Prispôsobiť okno"),
        ("Original", "Pôvodný"),
        ("Shrink", "Zmenšené"),
        ("Stretch", "Roztiahnuté"),
        ("Scrollbar", "Posuvník"),
        ("ScrollAuto", "Rolovať Auto"),
        ("Good image quality", "Dobrá kvalita obrazu"),
        ("Balanced", "Vyvážené"),
        ("Optimize reaction time", "Optimalizované pre čas odozvy"),
        ("Custom", "Vlastné"),
        ("Show remote cursor", "Zobrazovať vzdialený ukazovateľ myši"),
        ("Show quality monitor", "Zobraziť monitor kvality"),
        ("Disable clipboard", "Vypnúť schránku"),
        ("Lock after session end", "Po skončení uzamknúť plochu"),
        ("Insert", "Vložiť"),
        ("Insert Lock", "Uzamknúť"),
        ("Refresh", "Aktualizovať"),
        ("ID does not exist", "ID neexistuje"),
        ("Failed to connect to rendezvous server", "Nepodarilo sa pripojiť k zoznamovaciemu serveru"),
        ("Please try later", "Vyskúšajte neskôr"),
        ("Remote desktop is offline", "Vzdialená plocha nie je pripojená"),
        ("Key mismatch", "Kľúče sa nezhodujú"),
        ("Timeout", "Čas pre nadviazanie pripojenia vypršal"),
        ("Failed to connect to relay server", "Nepodarilo sa pripojiť k prepojovaciemu serveru"),
        ("Failed to connect via rendezvous server", "Nepodarilo sa pripojiť cez zoznamovací server"),
        ("Failed to connect via relay server", "Nepodarilo sa pripojiť cez prepojovací server"),
        ("Failed to make direct connection to remote desktop", "Nepodarilo sa nadviazať priamu komunikáciu so vzdialenou plochou"),
        ("Set Password", "Nastaviť heslo"),
        ("OS Password", "Heslo do operačného systému"),
        ("install_tip", "V niektorých prípadoch RustDesk nefunguje správne z dôvodu riadenia užívateľských oprávnení (UAC). Vyhnete sa tomu kliknutím na nižšie zobrazene tlačítko a nainštalovaním RuskDesk do systému."),
        ("Click to upgrade", "Kliknutím nainštalujete aktualizáciu"),
        ("Click to download", "Kliknutím potvrďte stiahnutie"),
        ("Click to update", "Kliknutím aktualizovať"),
        ("Configure", "Nastaviť"),
        ("config_acc", "Aby bolo možné na diaľku ovládať vašu plochu, je potrebné aplikácii RustDesk udeliť práva \"Dostupnosť\"."),
        ("config_screen", "Aby bolo možné na diaľku sledovať vašu obrazovku, je potrebné aplikácii RustDesk udeliť práva \"Zachytávanie obsahu obrazovky\"."),
        ("Installing ...", "Inštaluje sa"),
        ("Install", "Inštalovať"),
        ("Installation", "Inštalácia"),
        ("Installation Path", "Inštalačný adresár"),
        ("Create start menu shortcuts", "Vytvoriť zástupcu do ponuky Štart"),
        ("Create desktop icon", "Vytvoriť ikonu na ploche"),
        ("agreement_tip", "Spustením inštalácie prijímate licenčné podmienky."),
        ("Accept and Install", "Prijať a inštalovať"),
        ("End-user license agreement", "Licenčné podmienky dohodnuté s koncovým užívateľom"),
        ("Generating ...", "Generujem ..."),
        ("Your installation is lower version.", "Vaša inštalácia je staršia"),
        ("not_close_tcp_tip", "Nezatvárajte toto okno po celý čas, kedy používate TCP tunel"),
        ("Listening ...", "Čakám na pripojenie ..."),
        ("Remote Host", "Vzdialený počítač"),
        ("Remote Port", "Vzdialený port"),
        ("Action", "Akcia"),
        ("Add", "Pridať"),
        ("Local Port", "Lokálny port"),
        ("Local Address", "Lokálna adresa"),
        ("Change Local Port", "Zmena lokálneho portu"),
        ("setup_server_tip", "Pre zrýchlenie pripojenia si nainštalujte svoj vlastný server"),
        ("Too short, at least 6 characters.", "Príliš krátke, vyžaduje sa aspoň 6 znakov."),
        ("The confirmation is not identical.", "Potvrdenie nie je zhodné."),
        ("Permissions", "Práva"),
        ("Accept", "Prijať"),
        ("Dismiss", "Odmietnuť"),
        ("Disconnect", "Odpojiť"),
        ("Enable file copy and paste", "Povoliť kopírovanie a vkladanie súborov"),
        ("Connected", "Pripojené"),
        ("Direct and encrypted connection", "Priame a šifrované spojenie"),
        ("Relayed and encrypted connection", "Sprostredkované a šifrované spojenie"),
        ("Direct and unencrypted connection", "Priame a nešifrované spojenie"),
        ("Relayed and unencrypted connection", "Sprostredkované a nešifrované spojenie"),
        ("Enter Remote ID", "Zadajte ID vzdialenej plochy"),
        ("Enter your password", "Zadajte svoje heslo"),
        ("Logging in...", "Prihlasovanie sa...."),
        ("Enable RDP session sharing", "Povoliť zdieľanie RDP relácie"),
        ("Auto Login", "Automatické prihlásenie"),
        ("Enable direct IP access", "Povoliť priame pripojenie cez IP"),
        ("Rename", "Premenovať"),
        ("Space", "Medzera"),
        ("Create desktop shortcut", "Vytvoriť zástupcu na ploche"),
        ("Change Path", "Zmeniť adresár"),
        ("Create Folder", "Vytvoriť adresár"),
        ("Please enter the folder name", "Zadajte názov adresára"),
        ("Fix it", "Opraviť to"),
        ("Warning", "Upozornenie"),
        ("Login screen using Wayland is not supported", "Prihlasovacia obrazovka prostredníctvom Wayland nie je podporovaná"),
        ("Reboot required", "Vyžaduje sa reštart"),
        ("Unsupported display server", "Nepodporovaný zobrazovací (display) server"),
        ("x11 expected", "očakáva sa x11"),
        ("Port", "Port"),
        ("Settings", "Nastavenia"),
        ("Username", "Uživateľské meno"),
        ("Invalid port", "Neplatný port"),
        ("Closed manually by the peer", "Manuálne ukončené opačnou stranou pripojenia"),
        ("Enable remote configuration modification", "Povoliť zmeny konfigurácie zo vzdialeného PC"),
        ("Run without install", "Spustiť bez inštalácie"),
        ("Connect via relay", "Pripojenie prostredníctvom relay servera"),
        ("Always connect via relay", "Vždy pripájať cez prepájací server"),
        ("whitelist_tip", "Len vymenované IP adresy majú oprávnenie sa pripojiť k vzdialenej správe"),
        ("Login", "Prihlásenie"),
        ("Verify", "Overiť"),
        ("Remember me", "Zapamätať si"),
        ("Trust this device", "Dôverovať tomuto zariadeniu"),
        ("Verification code", "Overovací kód"),
        ("verification_tip", "Na vašu registrovanú e-mailovú adresu bol odoslaný overovací kód, zadajte ho a pokračujte v prihlasovaní."),
        ("Logout", "Odhlásenie"),
        ("Tags", "Štítky"),
        ("Search ID", "Hľadať ID"),
        ("whitelist_sep", "Oddelené čiarkou, bodkočiarkou, medzerou alebo koncom riadku"),
        ("Add ID", "Pridať ID"),
        ("Add Tag", "Pridať štítok"),
        ("Unselect all tags", "Zrušiť výber všetkých štítkov"),
        ("Network error", "Chyba siete"),
        ("Username missed", "Chýba užívateľské meno"),
        ("Password missed", "Chýba heslo"),
        ("Wrong credentials", "Nesprávne prihlasovacie údaje"),
        ("The verification code is incorrect or has expired", "Overovací kód je nesprávny alebo jeho platnosť vypršala"),
        ("Edit Tag", "Upraviť štítok"),
        ("Forget Password", "Zabudnúť heslo"),
        ("Favorites", "Obľúbené"),
        ("Add to Favorites", "Pridať medzi obľúbené"),
        ("Remove from Favorites", "Odstrániť z obľúbených"),
        ("Empty", "Prázdne"),
        ("Invalid folder name", "Neplatný názov adresára"),
        ("Socks5 Proxy", "Socks5 Proxy"),
        ("Socks5/Http(s) Proxy", "Socks5/Http(s) Proxy"),
        ("Discovered", "Objavené"),
        ("install_daemon_tip", "Ak chcete, aby sa spúšťal pri štarte systému, musíte nainštalovať systémovú službu."),
        ("Remote ID", "Vzdialené ID"),
        ("Paste", "Vložiť"),
        ("Paste here?", "Vložiť sem?"),
        ("Are you sure to close the connection?", "Ste si istý, že chcete ukončiť spojenie?"),
        ("Download new version", "Stiahnuť novú verziu"),
        ("Touch mode", "Dotykový režim"),
        ("Mouse mode", "Režim ovládania myšou"),
        ("One-Finger Tap", "Klepnutie jedným prstom"),
        ("Left Mouse", "Ľavé tlačidlo myši"),
        ("One-Long Tap", "Jedno dlhé klepnutie"),
        ("Two-Finger Tap", "Klepnutie dvoma prstami"),
        ("Right Mouse", "Pravé tlačidlo myši"),
        ("One-Finger Move", "Presúvanie jedným prstom"),
        ("Double Tap & Move", "Dvojité klepnutie a presun"),
        ("Mouse Drag", "Presun myšou"),
        ("Three-Finger vertically", "Pohyb tromi prstami zvisle"),
        ("Mouse Wheel", "Koliesko myši"),
        ("Two-Finger Move", "Pohyb dvoma prstami"),
        ("Canvas Move", "Pohyb zobrazenia"),
        ("Pinch to Zoom", "Roztiahnutím prstov priblížiť"),
        ("Canvas Zoom", "Priblíženie zobrazenia"),
        ("Reset canvas", "Obnoviť zobrazenie"),
        ("No permission of file transfer", "Prenos súborov nie je povolený"),
        ("Note", "Poznámka"),
        ("Connection", "Pripojenie"),
        ("Share Screen", "Zdielať obrazovku"),
        ("Chat", "Chat"),
        ("Total", "Celkom"),
        ("items", "položiek"),
        ("Selected", "Vybrané"),
        ("Screen Capture", "Snímanie obrazovky"),
        ("Input Control", "Ovládanie vstupných zariadení"),
        ("Audio Capture", "Snímanie zvuku"),
        ("File Connection", "Pripojenie súborov"),
        ("Screen Connection", "Pripojenie obrazu"),
        ("Do you accept?", "Súhlasíte?"),
        ("Open System Setting", "Otvorenie nastavení systému"),
        ("How to get Android input permission?", "Ako v systéme Android povoliť oprávnenie písať zo vstupného zariadenia?"),
        ("android_input_permission_tip1", "Aby bolo možné na diaľku ovládať vašu plochu pomocou myši alebo dotykov, je potrebné aplikácii RustDesk udeliť práva \"Dostupnosť\"."),
        ("android_input_permission_tip2", "Prejdite na stránku nastavení systému, nájdite a vstúpte do [Stiahnuté služby], zapnite [RustDesk Input] službu."),
        ("android_new_connection_tip", "Bola prijatá nová požiadavka na ovládanie vášho zariadenia."),
        ("android_service_will_start_tip", "Zapnutie \"Zachytávanie obsahu obrazovky\" automaticky spistí službu, čo iným zariadeniam umožní požiadať o pripojenie k tomuto zariadeniu."),
        ("android_stop_service_tip", "Zastavenie služby automaticky ukončí všetky naviazané spojenia."),
        ("android_version_audio_tip", "Vaša verzia Androidu neumožňuje zaznamenávanie zvuku. Prejdite na verziu Android 10 alebo vyššiu."),
        ("android_start_service_tip", "Ťuknutím na položku [Spustiť službu] alebo povolením povolenia [Snímanie obrazovky] spustite službu zdieľania obrazovky."),
        ("android_permission_may_not_change_tip", "Oprávnenia pre vytvorené pripojenia možno zmeniť až po opätovnom pripojení."),
        ("Account", "Účet"),
        ("Overwrite", "Prepísať"),
        ("This file exists, skip or overwrite this file?", "Preskočiť alebo prepísať existujúci súbor?"),
        ("Quit", "Ukončiť"),
        ("Help", "Nápoveda"),
        ("Failed", "Nepodarilo sa"),
        ("Succeeded", "Podarilo sa"),
        ("Someone turns on privacy mode, exit", "Niekto zapne režim súkromia, ukončite ho"),
        ("Unsupported", "Nepodporované"),
        ("Peer denied", "Peer poprel"),
        ("Please install plugins", "Nainštalujte si prosím pluginy"),
        ("Peer exit", "Peer exit"),
        ("Failed to turn off", "Nepodarilo sa vypnúť"),
        ("Turned off", "Vypnutý"),
        ("Language", "Jazyk"),
        ("Keep RustDesk background service", "Ponechať službu RustDesk na pozadí"),
        ("Ignore Battery Optimizations", "Ignorovať optimalizácie batérie"),
        ("android_open_battery_optimizations_tip", "Ak chcete túto funkciu vypnúť, prejdite na ďalšiu stránku nastavení RustDesku, vyhľadajte a zadajte položku [Batéria], zrušte začiarknutie položky [Neobmedzené]."),
        ("Start on boot", "Spustenie po štarte"),
        ("Start the screen sharing service on boot, requires special permissions", "Spustenie služby zdieľania obrazovky pri štarte systému, vyžaduje špeciálne oprávnenia"),
        ("Connection not allowed", "Spojenie nie je povolené"),
        ("Legacy mode", "Režim Legacy"),
        ("Map mode", "Režim mapovania"),
        ("Translate mode", "Režim prekladania"),
        ("Use permanent password", "Použitie trvalého hesla"),
        ("Use both passwords", "Používanie oboch hesiel"),
        ("Set permanent password", "Nastaviť trvalé heslo"),
        ("Enable remote restart", "Povoliť vzdialený reštart"),
        ("Restart remote device", "Reštartovať vzdialené zariadenie"),
        ("Are you sure you want to restart", "Ste si istý, že chcete reštartovať"),
        ("Restarting remote device", "Reštartovanie vzdialeného zariadenia"),
        ("remote_restarting_tip", "Vzdialené zariadenie sa reštartuje, zatvorte toto okno a po chvíli sa znovu pripojte pomocou trvalého hesla."),
        ("Copied", "Skopírované"),
        ("Exit Fullscreen", "Ukončiť celú obrazovku"),
        ("Fullscreen", "Celá obrazovka"),
        ("Mobile Actions", "Mobilné akcie"),
        ("Select Monitor", "Vyberte možnosť Monitor"),
        ("Control Actions", "Kontrolné akcie"),
        ("Display Settings", "Nastavenia displeja"),
        ("Ratio", "Pomer"),
        ("Image Quality", "Kvalita obrazu"),
        ("Scroll Style", "Štýl posúvania"),
        ("Show Toolbar", "Zobrazenie panela nástrojov"),
        ("Hide Toolbar", "Skrytie panela nástrojov"),
        ("Direct Connection", "Priame pripojenie"),
        ("Relay Connection", "Reléové pripojenie"),
        ("Secure Connection", "Zabezpečené pripojenie"),
        ("Insecure Connection", "Nezabezpečené pripojenie"),
        ("Scale original", "Pôvodná mierka"),
        ("Scale adaptive", "Prispôsobivá mierka"),
        ("General", "Všeobecné"),
        ("Security", "Zabezpečenie"),
        ("Theme", "Motív"),
        ("Dark Theme", "Tmavý motív"),
        ("Light Theme", "Svetlý motív"),
        ("Dark", "Tmavý"),
        ("Light", "Svetlý"),
        ("Follow System", "Podľa systému"),
        ("Enable hardware codec", "Povoliť hardwarový kodek"),
        ("Unlock Security Settings", "Odomknúť nastavenie zabezpečenia"),
        ("Enable audio", "Povoliť zvuk"),
        ("Unlock Network Settings", "Odomknúť nastavenie siete"),
        ("Server", "Server"),
        ("Direct IP Access", "Priamy IP prístup"),
        ("Proxy", "Proxy"),
        ("Apply", "Použiť"),
        ("Disconnect all devices?", "Odpojiť všetky zariadenia?"),
        ("Clear", "Zmazať"),
        ("Audio Input Device", "Vstupné zvukové zariadenie"),
        ("Use IP Whitelisting", "Použiť IP whitelisting"),
        ("Network", "Sieť"),
        ("Pin Toolbar", "Pripnúť panel nástrojov"),
        ("Unpin Toolbar", "Odpojiť panel nástrojov"),
        ("Recording", "Nahrávanie"),
        ("Directory", "Adresár"),
        ("Automatically record incoming sessions", "Automaticky nahrávať prichádzajúce relácie"),
        ("Change", "Zmeniť"),
        ("Start session recording", "Spustiť záznam relácie"),
        ("Stop session recording", "Zastaviť záznam relácie"),
        ("Enable recording session", "Povoliť nahrávanie relácie"),
        ("Enable LAN discovery", "Povolenie zisťovania siete LAN"),
        ("Deny LAN discovery", "Zakázať zisťovania siete LAN"),
        ("Write a message", "Napísať správu"),
        ("Prompt", "Výzva"),
        ("Please wait for confirmation of UAC...", "Počkajte, prosím, na potvrdenie UAC..."),
        ("elevated_foreground_window_tip", "Aktuálne okno vzdialenej plochy vyžaduje vyššie oprávnenia, takže dočasne nemôže používať myš a klávesnicu. Môžete požiadať vzdialeného používateľa, aby minimalizoval aktuálne okno, alebo kliknúť na tlačidlo povýšiť v okne správy pripojenia. Ak sa chcete vyhnúť tomuto problému, odporúčame nainštalovať softvér na vzdialené zariadenie."),
        ("Disconnected", "Odpojené"),
        ("Other", "Iné"),
        ("Confirm before closing multiple tabs", "Potvrdiť pred zatvorením viacerých kariet"),
        ("Keyboard Settings", "Nastavenia klávesnice"),
        ("Full Access", "Úplný prístup"),
        ("Screen Share", "Zdielanie obrazovky"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland vyžaduje Ubuntu 21.04 alebo vyššiu verziu."),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Wayland vyžaduje vyššiu verziu linuxovej distribúcie. Skúste X11 desktop alebo zmeňte OS."),
        ("JumpLink", "View"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Vyberte obrazovku, ktorú chcete zdieľať (Ovládajte na strane partnera)."),
        ("Show RustDesk", "Zobraziť RustDesk"),
        ("This PC", "Tento počítač"),
        ("or", "alebo"),
        ("Continue with", "Pokračovať s"),
        ("Elevate", "Zvýšiť"),
        ("Zoom cursor", "Kurzor priblíženia"),
        ("Accept sessions via password", "Prijímanie relácií pomocou hesla"),
        ("Accept sessions via click", "Prijímanie relácií kliknutím"),
        ("Accept sessions via both", "Prijímanie relácií prostredníctvom oboch"),
        ("Please wait for the remote side to accept your session request...", "Počkajte, kým vzdialená strana prijme vašu žiadosť o reláciu..."),
        ("One-time Password", "Jednorázové heslo"),
        ("Use one-time password", "Použiť jednorázové heslo"),
        ("One-time password length", "Dĺžka jednorázového hesla"),
        ("Request access to your device", "Žiadosť o prístup k vášmu zariadeniu"),
        ("Hide connection management window", "Skryť okno správy pripojenia"),
        ("hide_cm_tip", "Skrývanie povoľte len vtedy, ak relácie prijímate pomocou hesla a používate trvalé heslo."),
        ("wayland_experiment_tip", "Podpora Waylandu je v experimentálnej fáze, ak potrebujete bezobslužný prístup, použite X11."),
        ("Right click to select tabs", "Výber karty kliknutím pravým tlačidlom myši"),
        ("Skipped", "Vynechané"),
        ("Add to address book", "Pridať do adresára"),
        ("Group", "Skupina"),
        ("Search", "Vyhľadávanie"),
        ("Closed manually by web console", "Zatvorené ručne pomocou webovej konzoly"),
        ("Local keyboard type", "Typ lokálnej klávesnice"),
        ("Select local keyboard type", "Výber typu lokálnej klávesnice"),
        ("software_render_tip", "Ak používate grafickú kartu Nvidia v systéme Linux a vzdialené okno sa po pripojení okamžite zatvorí, môže vám pomôcť prepnutie na ovládač Nouveau s otvoreným zdrojovým kódom a výber softvérového vykresľovania. Vyžaduje sa softvérový reštart."),
        ("Always use software rendering", "Vždy použiť softvérové vykresľovanie"),
        ("config_input", "Ak chcete ovládať vzdialenú plochu pomocou klávesnice, musíte udeliť oprávnenie RustDesk \"Sledovanie vstupu\"."),
        ("config_microphone", "Ak chcete hovoriť na diaľku, musíte RustDesku udeliť povolenie \"Nahrávať zvuk\"."),
        ("request_elevation_tip", "Môžete tiež požiadať o zvýšenie, ak je niekto na vzdialenej strane."),
        ("Wait", "Počkajte"),
        ("Elevation Error", "Chyba navýšenia"),
        ("Ask the remote user for authentication", "Požiadať vzdialeného používateľa o overenie"),
        ("Choose this if the remote account is administrator", "Túto možnosť vyberte, ak je účet vzdialeného správcu"),
        ("Transmit the username and password of administrator", "Prenos používateľského mena a hesla správcu"),
        ("still_click_uac_tip", "Stále sa vyžaduje, aby vzdialený používateľ klikol na tlačidlo OK v okne UAC spusteného programu RustDesk."),
        ("Request Elevation", "Žiadosť o navýšenie"),
        ("wait_accept_uac_tip", "Počkajte, kým vzdialený používateľ prijme dialógové okno UAC."),
        ("Elevate successfully", "Úspešné navýšenie"),
        ("uppercase", "velké písmená"),
        ("lowercase", "malé písmená"),
        ("digit", "číslice"),
        ("special character", "špeciálny znak"),
        ("length>=8", "dĺžka>=8"),
        ("Weak", "Slabé"),
        ("Medium", "Stredné"),
        ("Strong", "Silné"),
        ("Switch Sides", "Prepínanie strán"),
        ("Please confirm if you want to share your desktop?", "Potvrďte, prosím, či chcete zdieľať svoju plochu?"),
        ("Display", "Obrazovka"),
        ("Default View Style", "Predvolený štýl zobrazenia"),
        ("Default Scroll Style", "Predvolený štýl posúvania"),
        ("Default Image Quality", "Predvolená kvalita obrazu"),
        ("Default Codec", "Predvolený kodek"),
        ("Bitrate", "Dátový tok"),
        ("FPS", "FPS"),
        ("Auto", "Auto"),
        ("Other Default Options", "Ďalšie predvolené možnosti"),
        ("Voice call", "Hlasový hovor"),
        ("Text chat", "Textový chat"),
        ("Stop voice call", "Zastaviť hlasový hovor"),
        ("relay_hint_tip", "Priame pripojenie nemusí byť možné, môžete sa pokúsiť pripojiť prostredníctvom presmerovacieho servera. Okrem toho, ak chcete pri prvom pokuse použiť presmerovací server, môžete k ID pridať príponu \"/r\" alebo vybrať možnosť \"Vždy sa pripájať cez bránu\" na karte posledných relácií, ak existuje."),
        ("Reconnect", "Znovu pripojiť"),
        ("Codec", "Kodek"),
        ("Resolution", "Rozlíšenie"),
        ("No transfers in progress", "Žiadne prebiehajúce presuny"),
        ("Set one-time password length", "Nastaviť dĺžku jednorazového hesla"),
        ("RDP Settings", "Nastavenia RDP"),
        ("Sort by", "Usporiadať podľa"),
        ("New Connection", "Nové pripojenie"),
        ("Restore", "Obnoviť"),
        ("Minimize", "Minimalizovať"),
        ("Maximize", "Maximalizovať"),
        ("Your Device", "Vaše zariadenie"),
        ("empty_recent_tip", "Ups, žiadna nedávna relácia!\nČas naplánovať novú."),
        ("empty_favorite_tip", "Ešte nemáte obľúbeného partnera?\nNájdite niekoho, s kým sa môžete spojiť, a pridajte si ho do obľúbených!"),
        ("empty_lan_tip", "Ale nie, zdá sa, že sme zatiaľ neobjavili žiadnu protistranu."),
        ("empty_address_book_tip", "Ach bože, zdá sa, že vo vašom adresári momentálne nie sú uvedení žiadni kolegovia."),
        ("eg: admin", "napr. admin"),
        ("Empty Username", "Prázdne používateľské meno"),
        ("Empty Password", "Prázdne heslo"),
        ("Me", "Ja"),
        ("identical_file_tip", "Tento súbor je identický so súborom partnera."),
        ("show_monitors_tip", "Zobraziť monitory na paneli nástrojov"),
        ("View Mode", "Režim zobrazenia"),
        ("login_linux_tip", "Ak chcete povoliť reláciu Desktop X, musíte sa prihlásiť do vzdialeného konta Linuxu."),
        ("verify_rustdesk_password_tip", "Overenie hesla RustDesk"),
        ("remember_account_tip", "Zapamätať si tento účet"),
        ("os_account_desk_tip", "Toto konto sa používa na prihlásenie do vzdialeného operačného systému a na povolenie relácie pracovnej plochy v režime headless."),
        ("OS Account", "Účet operačného systému"),
        ("another_user_login_title_tip", "Ďalší používateľ je už prihlásený"),
        ("another_user_login_text_tip", "Odpojiť"),
        ("xorg_not_found_title_tip", "Xorg nebol nájdený"),
        ("xorg_not_found_text_tip", "Prosím, nainštalujte Xorg"),
        ("no_desktop_title_tip", "Nie je k dispozícii žiadna plocha"),
        ("no_desktop_text_tip", "Nainštalujte si prostredie GNOME"),
        ("No need to elevate", "Navýšenie nie je potrebné"),
        ("System Sound", "Systémový zvuk"),
        ("Default", "Predvolené"),
        ("New RDP", "Nové RDP"),
        ("Fingerprint", "Odtlačok prsta"),
        ("Copy Fingerprint", "Kopírovať odtlačok prsta"),
        ("no fingerprints", "žiadne odtlačky prstov"),
        ("Select a peer", "Výber partnera"),
        ("Select peers", "Výber partnerov"),
        ("Plugins", "Pluginy"),
        ("Uninstall", "Odinštalovať"),
        ("Update", "Aktualizovať"),
        ("Enable", "Povoliť"),
        ("Disable", "Zakázať"),
        ("Options", "Možnosti"),
        ("resolution_original_tip", "Pôvodné rozlíšenie"),
        ("resolution_fit_local_tip", "Prispôsobiť miestne rozlíšenie"),
        ("resolution_custom_tip", "Vlastné rozlíšenie"),
        ("Collapse toolbar", "Zbaliť panel nástrojov"),
        ("Accept and Elevate", "Prijať navýšenie"),
        ("accept_and_elevate_btn_tooltip", "Prijmite pripojenie a zvýšte oprávnenia UAC."),
        ("clipboard_wait_response_timeout_tip", "Čas na čakanie na kópiu odpovede uplynul."),
        ("Incoming connection", "Prichádzajúce pripojenie"),
        ("Outgoing connection", "Odchádzajúce pripojenie"),
        ("Exit", "Ukončiť"),
        ("Open", "Otvoriť"),
        ("logout_tip", "Naozaj sa chcete odhlásiť?"),
        ("Service", "Služba"),
        ("Start", "Spustiť"),
        ("Stop", "Zastaviť"),
        ("exceed_max_devices", "Dosiahli ste maximálny počet spravovaných zariadení."),
        ("Sync with recent sessions", "Synchronizovať s poslednými reláciami"),
        ("Sort tags", "Zoradiť štítky"),
        ("Open connection in new tab", "Otvoriť pripojenie v novej karte"),
        ("Move tab to new window", "Presunúť kartu do nového okna"),
        ("Can not be empty", "Nemôže byť prázdne"),
        ("Already exists", "Už existuje"),
        ("Change Password", "Zmeniť heslo"),
        ("Refresh Password", "Obnoviť heslo"),
        ("ID", "ID"),
        ("Grid View", "Mriežka"),
        ("List View", "Zoznam"),
        ("Select", "Vybrať"),
        ("Toggle Tags", "Prepnúť štítky"),
        ("pull_ab_failed_tip", "Nepodarilo sa obnoviť adresár"),
        ("push_ab_failed_tip", "Nepodarilo sa synchronizovať adresár so serverom"),
        ("synced_peer_readded_tip", "Zariadenia, ktoré boli prítomné v posledných reláciách, budú synchronizované späť do adresára."),
        ("Change Color", "Zmeniť farbu"),
        ("Primary Color", "Hlavná farba"),
        ("HSV Color", "HSV farba"),
        ("Installation Successful!", "Inštalácia úspešná!"),
        ("Installation failed!", "Inštalácia zlyhala!"),
        ("Reverse mouse wheel", "Reverzné koliesko myši"),
        ("{} sessions", "{} relácií"),
        ("scam_title", "Možno ste boli oklamaní!"),
        ("scam_text1", "Ak telefonujete s niekým, koho nepoznáte a komu nedôverujete a kto vás požiadal o použitie a spustenie aplikácie RustDesk, nepokračujte v hovore a okamžite zaveste."),
        ("scam_text2", "Pravdepodobne ide o podvodníka, ktorý sa snaží ukradnúť vaše peniaze alebo iné súkromné informácie."),
        ("Don't show again", "Nezobrazovať znova"),
        ("I Agree", "Súhlasím"),
        ("Decline", "Odmietnuť"),
        ("Timeout in minutes", "Časový limit v minútach"),
        ("auto_disconnect_option_tip", "Automatické ukončenie prichádzajúcich relácií, keď je používateľ nečinný"),
        ("Connection failed due to inactivity", "Pripojenie zlyhalo z dôvodu nečinnosti"),
        ("Check for software update on startup", "Kontrola aktualizácií softvéru pri spustení"),
        ("upgrade_rustdesk_server_pro_to_{}_tip", "Aktualizujte RustDesk Server Pro na verziu {} alebo novšiu!"),
        ("pull_group_failed_tip", "Nepodarilo sa obnoviť skupinu"),
        ("Filter by intersection", "Filtrovať podľa križovatky"),
        ("Remove wallpaper during incoming sessions", "Odstrániť tapetu počas prichádzajúcich relácií"),
        ("Test", "Test"),
        ("display_is_plugged_out_msg", "Obrazovka je odpojená, prepnite na prvú obrazovku."),
        ("No displays", "Žiadne obrazovky"),
        ("elevated_switch_display_msg", "Prepnite na primárnu obrazovku, pretože viacero obrazoviek nie je podporovaných vo zvýšenom režime."),
        ("Open in new window", "Otvoriť v novom okne"),
        ("Show displays as individual windows", "Zobraziť obrazovky ako jednotlivé okná"),
        ("Use all my displays for the remote session", "Použiť všetky moje obrazovky pre vzdialenú reláciu"),
        ("selinux_tip", "Na vašom zariadení je povolený SELinux, čo môže brániť správnemu spusteniu RustDesku ako spravovanej strany."),
        ("Change view", "Zmeniť pohľad"),
        ("Big tiles", "Veľké dlaždice"),
        ("Small tiles", "Malé dlaždice"),
        ("List", "Zoznam"),
        ("Virtual display", "Virtuálny displej"),
        ("Plug out all", "Odpojiť všetky"),
        ("True color (4:4:4)", "Skutočná farba (4:4:4)"),
        ("Enable blocking user input", "Povoliť blokovanie vstupu od používateľa"),
        ("id_input_tip", "Môžete zadať ID, priamu IP adresu alebo doménu s portom (<doména>:<port>).\nAk chcete získať prístup k zariadeniu na inom serveri, doplňte adresu servera (<id>@<adresa_servera>?key=<hodnota_kľúča>), napríklad,\n9123456234@192.168.16.1:21117?key=5Qbwsde3unUcJBtrx9ZkvUmwFNoExHzpryHuPUdqlWM=.\nAk chcete získať prístup k zariadeniu na verejnom serveri, zadajte \"<id>@public\", kľúč nie je potrebný pre verejný server."),
        ("privacy_mode_impl_mag_tip", "Režim 1"),
        ("privacy_mode_impl_virtual_display_tip", "Režim 2"),
        ("Enter privacy mode", "Vstup do režimu súkromia"),
        ("Exit privacy mode", "Ukončiť režim súkromia"),
        ("idd_not_support_under_win10_2004_tip", "Ovládač nepriameho zobrazenia nie je podporovaný. Vyžaduje sa systém Windows 10, verzia 2004 alebo novšia."),
        ("switch_display_elevated_connections_tip", "Prepínanie na inú ako primárnu obrazovku nie je podporované vo zvýšenom režime, ak existuje viacero pripojení. Ak chcete ovládať viacero obrazoviek, skúste to po inštalácii znova."),
        ("input_source_1_tip", "Vstupný zdroj 1"),
        ("input_source_2_tip", "Vstupný zdroj 2"),
        ("capture_display_elevated_connections_tip", "Snímanie viacerých displejov nie je podporované v režime privilegovaného používateľa. Ak chcete ovládať viac displejov, skúste to po inštalácii znova."),
        ("Swap control-command key", "Vymeniť kláves ovládania a príkazu"),
        ("swap-left-right-mouse", "Prehodiť ľavé a pravé tlačidlo myši"),
        ("2FA code", "2FA kód"),
        ("More", "Viac"),
        ("enable-2fa-title", "Povoliť dvojfaktorové overenie"),
        ("enable-2fa-desc", "Prosím, nastavte si svoj autentifikátor. Na svojom telefóne alebo počítači môžete použiť autentifikačnú aplikáciu, ako je Authy, Microsoft alebo Google Authenticator.\n\nNaskenujte QR kód pomocou svojej aplikácie a zadajte kód, ktorý aplikácia zobrazí, aby ste povolili dvojfaktorové overenie."),
        ("wrong-2fa-code", "Kód sa nepodarilo overiť. Skontrolujte, či sú nastavenia kódu a miestneho času správne"),
        ("enter-2fa-title", "Dvojfaktorové overenie"),
        ("Email verification code must be 6 characters.", "Overovací kód e-mailu musí mať 6 znakov."),
        ("2FA code must be 6 digits.", "Kód 2FA musí obsahovať 6 číslic."),
        ("Multiple Windows sessions found", "Našlo sa viacero relácií systému Windows"),
        ("Please select the session you want to connect to", "Vyberte reláciu, ku ktorej sa chcete pripojiť"),
        ("powered_by_me", "Poháňané aplikáciou RustDesk"),
        ("outgoing_only_desk_tip", "Toto je prispôsobené vydanie.\nMôžete sa pripojiť k iným zariadeniam, ale iné zariadenia sa k vášmu zariadeniu pripojiť nemôžu."),
        ("preset_password_warning", "Toto prispôsobené vydanie sa dodáva s prednastaveným heslom. Každý, kto pozná toto heslo, môže získať plnú kontrolu nad vaším zariadením. Ak ste to neočakávali, okamžite softvér odinštalujte."),
        ("Security Alert", "Bezpečnostné upozornenie"),
        ("My address book", "Môj adresár"),
        ("Personal", "Osobné"),
        ("Owner", "Vlastník"),
        ("Set shared password", "Nastaviť zdieľané heslo"),
        ("Exist in", "Existovať v"),
        ("Read-only", "len na čítanie"),
        ("Read/Write", "Režim čítania/zápisu"),
        ("Full Control", "Úplná kontrola"),
        ("share_warning_tip", "Vyššie uvedené polia sú zdieľané a viditeľné pre ostatných."),
        ("Everyone", "Každý"),
        ("ab_web_console_tip", "Viac na webovej konzole"),
        ("allow-only-conn-window-open-tip", "Povoliť pripojenie iba vtedy, ak je otvorené okno aplikácie RustDesk"),
        ("no_need_privacy_mode_no_physical_displays_tip", "Žiadne fyzické displeje, nie je potrebné používať režim ochrany osobných údajov."),
        ("Follow remote cursor", "Nasledovať vzdialený kurzor"),
        ("Follow remote window focus", "Nasledovať vzdialené zameranie okna"),
        ("default_proxy_tip", ""),
    ].iter().cloned().collect();
}
