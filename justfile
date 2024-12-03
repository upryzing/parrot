publish:
    cargo publish --package upryzing-config
    cargo publish --package upryzing-result
    cargo publish --package upryzing-files
    cargo publish --package upryzing-permissions
    cargo publish --package upryzing-models
    cargo publish --package upryzing-presence
    cargo publish --package upryzing-database

patch:
    cargo release version patch --execute

minor:
    cargo release version minor --execute

major:
    cargo release version major --execute

release:
    scripts/try-tag-and-release.sh
