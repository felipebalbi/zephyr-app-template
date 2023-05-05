export LS_OPTIONS='-F --color=auto'
alias ls='ls $LS_OPTIONS'
if [ "${CODESPACES}" = "true" ]; then
  export WORKSPACE_DIR="$HOME/workspaces/zephyr-app-template"
fi
if [ -f "$WORKSPACE_DIR/zephyr/zephyr-env.sh" ]; then
  source "$WORKSPACE_DIR/zephyr/zephyr-env.sh"
fi
