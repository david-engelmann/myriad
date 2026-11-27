# PowerShell completion for myriad
Register-ArgumentCompleter -Native -CommandName myriad -ScriptBlock {
  param($wordToComplete, $commandAst, $cursorPosition)
  $subcommands = @('map', 'check', 'claims', 'census', 'atlas', 'index', 'changelog', 'lineage', 'returns', 'standing', 'absence', 'arrivals', 'departures', 'matched', 'runs', 'presence', 'standings', 'company', 'sets', 'rosters', 'fellows', '-h', '--help')
  $elements = $commandAst.CommandElements
  if ($elements.Count -eq 1 -or ($elements.Count -eq 2 -and $wordToComplete.Length -gt 0)) {
    $subcommands | Where-Object { $_ -like "$wordToComplete*" } | ForEach-Object {
      [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterValue', $_)
    }
    return
  }
  if ($elements.Count -ge 2 -and $wordToComplete.StartsWith('-')) {
    @('-h', '--help') | Where-Object { $_ -like "$wordToComplete*" } | ForEach-Object {
      [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterName', $_)
    }
    return
  }
}
