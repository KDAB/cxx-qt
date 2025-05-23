#!/bin/sh

set -e

echo "Recreating KDBindings headers..."
for header in $(fd ".*.h" */); do
  # Ensure we don't try to process *.h if no actual header file exists.
  [ -f "$header" ] || continue

  base=$(basename "$header")
  echo "Creating $base"
  echo "#include \"$header\"" > "$base"
done


