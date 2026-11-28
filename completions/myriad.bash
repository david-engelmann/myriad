# bash completion for myriad
_myriad() {
  local cur prev
  COMPREPLY=()
  cur="${COMP_WORDS[COMP_CWORD]}"
  prev="${COMP_WORDS[COMP_CWORD-1]}"
  if [[ $COMP_CWORD -eq 1 ]]; then
    COMPREPLY=( $(compgen -W "map check claims census atlas index changelog lineage returns standing absence arrivals departures matched runs presence standings company sets rosters fellows -h --help" -- "$cur") )
    return 0
  fi
  if [[ "$cur" == -* ]]; then
    COMPREPLY=( $(compgen -W "-h --help" -- "$cur") )
    return 0
  fi
  command -v compopt >/dev/null 2>&1 && compopt -o filenames
  local line
  while IFS= read -r line; do
    COMPREPLY+=( "$line" )
  done < <(compgen -f -- "$cur")
  return 0
}
complete -F _myriad myriad
