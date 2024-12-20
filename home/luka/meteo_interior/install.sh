#!/bin/bash

# Vérifie si on est en root
if [ "$EUID" -ne 0 ]; then 
    echo "Ce script doit être exécuté en tant que root"
    exit 1
fi

# Crée le répertoire d'installation
mkdir -p /opt/meteo_interior

# Copie l'exécutable
cp ./target/release/meteo_interior /opt/meteo_interior/

# Crée le fichier de configuration
cat > /opt/meteo_interior/config.env << EOL
SERVER_URL=http://server_api_address:server_api_port
SENSOR_LOCATION=interior
READ_INTERVAL_SECS=60
RUST_LOG=info
EOL

# Ajuste les permissions
chown -R root:root /opt/meteo_interior
chmod 755 /opt/meteo_interior
chmod 644 /opt/meteo_interior/config.env
chmod 755 /opt/meteo_interior/meteo_interior

# Copie et active le service systemd
cp meteo_interior.service /etc/systemd/system/
systemctl daemon-reload
systemctl enable meteo_interior
systemctl start meteo_interior

echo "Installation terminée. Vérifiez le statut avec:"
echo "systemctl status meteo_interior"