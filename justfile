_default:
  @just --list

dev:
  moon client:tauri

migrate-up:
  sea-orm-cli migrate up -d crates/migration

migrate-down n:
  sea-orm-cli migrate down -n {{n}} -d crates/migration

generate-entity:
  sea-orm-cli generate entity -o crates/entity/src/entities
