# 🌡️ Meteo Interior/Exterior

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Raspberry Pi](https://img.shields.io/badge/Raspberry%20Pi-C51A4A?style=flat&logo=Raspberry-Pi)](https://www.raspberrypi.org/)
[![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)

Un système de surveillance météorologique basé sur le capteur DHT11 pour Raspberry Pi, permettant de mesurer la température et l'humidité en intérieur ou extérieur.

## 📋 Fonctionnalités

- Lecture de température (°C)
- Lecture d'humidité (%)
- Envoi automatique des données vers un serveur API
- Installation en tant que service système
- Gestion des erreurs et tentatives de reconnexion
- Configuration flexible via variables d'environnement

## 🛠️ Prérequis

- Raspberry Pi
- Capteur DHT11
- Rust installé sur le système
- Accès au GPIO (droits root)

## 📦 Installation

1. Clonez le dépôt :
```bash
git clone https://github.com/votre-username/meteo_interior.git
cd meteo_interior
```

2. Compilez le projet :
```bash
cargo build --release
```

3. Exécutez le script d'installation en tant que root :
```bash
sudo ./install.sh
```

## ⚙️ Configuration

Le service utilise un fichier de configuration situé dans `/opt/meteo_interior/config.env`. Voici les variables disponibles :

| Variable | Description | Valeur par défaut |
|----------|-------------|-------------------|
| SERVER_URL | URL de l'API serveur | http://localhost:8080 |
| SENSOR_LOCATION | Emplacement du capteur (interior/exterior) | interior |
| READ_INTERVAL_SECS | Intervalle de lecture en secondes | 600 |
| RUST_LOG | Niveau de log | info |

## 📡 Utilisation

Le service démarre automatiquement après l'installation. Vous pouvez gérer le service avec les commandes systemd :

```bash
# Vérifier l'état
sudo systemctl status meteo_interior

# Redémarrer
sudo systemctl restart meteo_interior

# Arrêter
sudo systemctl stop meteo_interior

# Désactiver au démarrage
sudo systemctl disable meteo_interior
```

## 🔌 Connexion du DHT11

Connectez le capteur DHT11 au Raspberry Pi selon le schéma suivant :
- VCC → 3.3V (Pin 1)
- GND → Ground (Pin 6)
- DATA → GPIO4 (Pin 7)

## 📊 Format des données

Les données sont envoyées au serveur au format JSON :

```json
{
  "temperature": {
    "value": 23.5,
    "unit": "°C"
  },
  "humidity": {
    "value": 45.0,
    "unit": "%"
  },
  "location": "interior"
}
```

## 🔧 Dépannage

1. **Le service ne démarre pas**
   - Vérifiez les logs : `journalctl -u meteo_interior`
   - Vérifiez les permissions des fichiers dans `/opt/meteo_interior/`

2. **Erreurs de lecture du capteur**
   - Vérifiez le câblage
   - Assurez-vous que le GPIO4 est bien utilisé
   - Vérifiez les droits d'accès au GPIO

3. **Erreurs de connexion au serveur**
   - Vérifiez l'URL du serveur dans la configuration
   - Assurez-vous que le serveur est accessible
   - Vérifiez les pare-feu éventuels

## 📄 Licence

Ce projet est sous licence GNU GPL v3 - voir le fichier [LICENSE](LICENSE) pour plus de détails.

## 🤝 Contribution

Les contributions sont les bienvenues ! N'hésitez pas à ouvrir une issue ou soumettre une pull request.

---

Développé avec ❤️ par Luka Chassaing