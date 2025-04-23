#!/bin/sh

GRAFANA_DIR="./grafana"

check_permissions() {
  current_perms=$(stat -c "%a" "$GRAFANA_DIR")
  echo "$current_perms"
}

fix_permissions_if_needed() {
  current_perms=$(check_permissions)

  if [ "$current_perms" != "755" ]; then
    echo "Current permissions are $current_perms. Updating to 755..."
    chmod 755 "$GRAFANA_DIR"
  else
    echo "Permissions are already correct (755)."
  fi
}

run_docker_compose() {
  echo "Running Docker Compose..."
  docker compose up --build
}

main() {
  fix_permissions_if_needed
  run_docker_compose
}

main
