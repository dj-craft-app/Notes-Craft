#!/usr/bin/env python3
"""
Script amélioré pour générer des issues GitHub depuis roadmap_v3.md

Conformément à AGENTS.md:
- UNE ISSUE PAR ÉTAPE TESTABLE
- Contient: phase, étape, critères d'acceptation, dépendances
- Labels: phase-X, priority:Y
"""

import re
import subprocess
import sys
import json
from typing import List, Dict, Optional


class IssueGenerator:
    def __init__(self, roadmap_path: str):
        self.roadmap_path = roadmap_path
        self.phases = {}
        self.current_phase = None
        self.current_phase_id = None
        self.current_section = None  # 'main' or 'sub' (for 0.13)
        
    def parse_roadmap(self):
        """Parse roadmap_v3.md pour extraire les phases et étapes"""
        with open(self.roadmap_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        lines = content.split('\n')
        
        for i, line in enumerate(lines):
            # Match Phase header (e.g., "### Phase 1: Core System")
            phase_match = re.match(r'^### Phase (\d+): (.+)', line)
            if phase_match:
                self.current_phase_id = phase_match.group(1)
                self.current_phase_name = phase_match.group(2).split('[')[0].strip()
                self.current_section = 'main'
                self.phases[self.current_phase_id] = {
                    'name': self.current_phase_name,
                    'objective': '',
                    'steps': [],
                    'criteria': '',
                    'dependencies': ''
                }
                continue
            
            # Match Sub-phase (e.g., "#### 0.13 PoC Moteur Datalog")
            sub_phase_match = re.match(r'^#### (\d+\.\d+) (.+)', line)
            if sub_phase_match and self.current_phase_id:
                step_id = sub_phase_match.group(1)
                step_name = sub_phase_match.group(2).split('[')[0].strip()
                # Create a pseudo-phase for this
                self.current_section = 'sub'
                self.phases[f"{self.current_phase_id}-{step_id}"] = {
                    'name': f"{step_name} (Sub-phase of {self.current_phase_id})",
                    'objective': '',
                    'steps': [],
                    'criteria': '',
                    'dependencies': ''
                }
                continue
            
            # Stop collecting when we hit the next phase
            if self.current_phase_id and line.startswith('### Phase') and not line.startswith(f'### Phase {self.current_phase_id}'):
                self.current_phase_id = None
            
            if self.current_phase_id:
                # Match Objective
                if line.startswith('**Objectif:**'):
                    self.phases[self.current_phase_id]['objective'] = line.replace('**Objectif:**', '').strip()
                    continue
                
                # Match Dependencies
                if line.startswith('**Dependances:**'):
                    deps = line.replace('**Dependances:**', '').strip()
                    # Get all consecutive dependency lines
                    j = i + 1
                    while j < len(lines) and lines[j].strip() and not lines[j].startswith('**') and not lines[j].startswith('---'):
                        deps += '\n' + lines[j].strip()
                        j += 1
                    self.phases[self.current_phase_id]['dependencies'] = deps
                    continue
                
                # Match Criteria section
                if line.startswith('#### Critere d\'acceptation') or line.startswith('#### Critères d\'acceptation'):
                    # Collect criteria lines until next section
                    criteria_lines = []
                    for j in range(i+1, len(lines)):
                        if lines[j].startswith('####') or lines[j].startswith('---') or lines[j].startswith('###'):
                            break
                        if lines[j].strip() and not lines[j].startswith('|') and not lines[j].startswith('**'):
                            criteria_lines.append(lines[j].strip())
                    self.phases[self.current_phase_id]['criteria'] = '\n'.join(criteria_lines).strip()
                    continue
                
                # Match Steps table
                if line.startswith('| ID | Etape |') or line.startswith('| ID | Step |'):
                    # Parse the table rows
                    step_pattern = re.compile(r'^\| (\d+\.\d+) \| (.+?) \| (.+?) \| (.+?) \| (.+?) \| (.+?) \|$')
                    
                    for j in range(i+1, len(lines)):
                        if not lines[j].startswith('|'):
                            break
                        
                        step_match = step_pattern.match(lines[j])
                        if step_match:
                            step_id = step_match.group(1)
                            step_name = step_match.group(2)
                            priority = step_match.group(3)
                            estimation = step_match.group(4)
                            dependencies = step_match.group(5)
                            source = step_match.group(6)
                            
                            self.phases[self.current_phase_id]['steps'].append({
                                'id': step_id,
                                'name': step_name,
                                'priority': priority,
                                'estimation': estimation,
                                'dependencies': dependencies,
                                'source': source
                            })
        
        return self.phases
    
    def get_priority_label(self, priority: str) -> str:
        """Convertit la priorité en label"""
        priority_lower = priority.lower()
        if 'critique' in priority_lower:
            return 'priority:critical'
        elif 'haut' in priority_lower or 'high' in priority_lower:
            return 'priority:high'
        elif 'moyen' in priority_lower or 'medium' in priority_lower:
            return 'priority:medium'
        else:
            return 'priority:low'
    
    def generate_issue(self, phase_id: str, step: Dict) -> Dict:
        """Génère une issue complète"""
        phase = self.phases[phase_id]
        
        # Extract main phase ID (remove sub-phase suffix if present)
        main_phase_id = phase_id.split('-')[0] if '-' in phase_id else phase_id
        
        title = f"{step['id']}: {step['name']}"
        
        body = f"""# {step['id']}: {step['name']}

## Phase
**Phase {main_phase_id}: {phase['name']}**

## Objectif de la Phase
{phase['objective']}

## Étape
**{step['id']}** - {step['name']}

### Priorité
{step['priority']}

### Estimation
{step['estimation']}

### Dépendances
{step['dependencies']}

### Source
{step['source']}

## Critères d'Acceptation
{phase['criteria']}

---

### Dépendances de la Phase
{phase['dependencies']}

### Labels
phase-{main_phase_id},{self.get_priority_label(step['priority'])}"""
        
        labels = [f"phase-{main_phase_id}", self.get_priority_label(step['priority'])]
        
        return {
            'title': title,
            'body': body,
            'labels': labels
        }
    
    def create_github_issue(self, issue_data: Dict) -> bool:
        """Crée une issue GitHub via gh CLI"""
        title = issue_data['title']
        body = issue_data['body']
        labels = ','.join(issue_data['labels'])
        
        print(f"Création de l'issue: {title}")
        print(f"  Labels: {labels}")
        
        # Écrire le body dans un fichier temporaire pour éviter les problèmes d'échappement
        import tempfile
        import os
        
        with tempfile.NamedTemporaryFile(mode='w', suffix='.md', delete=False) as f:
            f.write(body)
            temp_file = f.name
        
        try:
            cmd = [
                'gh', 'issue', 'create',
                '--title', title,
                '--body-file', temp_file,
                '--label', labels
            ]
            
            result = subprocess.run(cmd, capture_output=True, text=True, check=False)
            
            if result.returncode == 0:
                print(f"  ✓ Issue créée avec succès")
                # Extraire le numéro de l'issue
                match = re.search(r'#(\d+)', result.stdout)
                if match:
                    print(f"  Issue #{match.group(1)}")
                return True
            else:
                print(f"  ✗ Erreur: {result.stderr}")
                if result.stdout:
                    print(f"  Output: {result.stdout}")
                return False
        finally:
            os.unlink(temp_file)
    
    def get_existing_issues(self) -> List[Dict]:
        """Récupère les issues existantes"""
        try:
            result = subprocess.run(
                ['gh', 'issue', 'list', '--state', 'open', '--json', 'number,title,labels'],
                capture_output=True, text=True, check=True
            )
            issues = json.loads(result.stdout)
            return issues
        except Exception as e:
            print(f"Erreur lors de la récupération des issues: {e}")
            return []
    
    def issue_exists(self, step_id: str, existing_issues: List[Dict]) -> bool:
        """Vérifie si une issue existe déjà pour cette étape"""
        for issue in existing_issues:
            title = issue.get('title', '')
            if step_id in title:
                return True
        return False
    
    def run(self, dry_run: bool = True, phase_filter: Optional[str] = None):
        """Exécute la génération des issues"""
        self.parse_roadmap()
        
        print(f"Phases trouvées: {list(self.phases.keys())}")
        print(f"Nombre total d'étapes: {sum(len(p['steps']) for p in self.phases.values())}")
        
        # Récupérer les issues existantes
        existing_issues = self.get_existing_issues()
        print(f"Issues existantes: {len(existing_issues)}")
        
        for phase_id, phase in self.phases.items():
            if phase_filter and phase_id != phase_filter and not phase_id.startswith(phase_filter):
                continue
                
            if not phase['steps']:
                continue
            
            print(f"\n=== Phase {phase_id}: {phase['name']} ===")
            print(f"Objectif: {phase['objective']}")
            print(f"Étapes: {len(phase['steps'])}")
            
            for step in phase['steps']:
                # Vérifier si l'issue existe déjà
                if self.issue_exists(step['id'], existing_issues):
                    print(f"  [EXISTE] {step['id']}: {step['name']}")
                    continue
                
                issue_data = self.generate_issue(phase_id, step)
                
                if not dry_run:
                    self.create_github_issue(issue_data)
                else:
                    print(f"  [NOUVELLE] {step['id']}: {step['name']}")
                    print(f"    Labels: {issue_data['labels']}")


if __name__ == '__main__':
    dry_run = '--dry-run' not in sys.argv
    phase_filter = None
    
    if '--phase' in sys.argv:
        idx = sys.argv.index('--phase')
        if idx + 1 < len(sys.argv):
            phase_filter = sys.argv[idx + 1]
    
    generator = IssueGenerator('/Users/amaury/Documents/REBUILD APPS/Note-Craft/docs/roadmap/roadmap_v3.md')
    generator.run(dry_run=dry_run, phase_filter=phase_filter)
