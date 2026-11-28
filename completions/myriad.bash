# bash completion for myriad
_myriad() {
  local cur
  COMPREPLY=()
  cur="${COMP_WORDS[COMP_CWORD]}"
  if [[ $COMP_CWORD -eq 1 ]]; then
    COMPREPLY=( $(compgen -W "map check claims census atlas index changelog lineage returns standing absence arrivals departures matched runs presence standings company sets rosters fellows -h --help" -- "$cur") )
    return 0
  fi
  if [[ "$cur" == -* ]]; then
    COMPREPLY=( $(compgen -W "-h --help" -- "$cur") )
    return 0
  fi
  return 0
}
complete -o bashdefault -o default -F _myriad myriad
