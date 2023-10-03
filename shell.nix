(builtins.getFlake
  ("git+file://" + toString ./.)).devShells.${builtins.currentSystem}.ci
