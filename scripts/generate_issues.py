#!/usr/bin/env python3
"""
Script pour générer des issues GitHub depuis roadmap_v3.md

Conformément à AGENTS.md:
- UNE ISSUE PAR ÉTAPE TESTABLE
- Contient: phase, étape, critères d'acceptation, dépendances
- Labels: phase-X
- Priorité = ordre des phases
"""

import re
import subprocess
import sys
from typing import List, Dict, Optional


class IssueGenerator:
    def __init__(self, roadmap_path: str):
        self.roadmap_path = roadmap_path
        self.phases = {}
        self.current_phase = None
        self.current_phase_id = None
        
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
                self.current_phase_name = phase_match.group(2)
                self.phases[self.current_phase_id] = {
                    'name': self.current_phase_name,
                    'objective': '',
                    'steps': [],
                    'criteria': '',
                    'dependencies': ''
                }
                continue
            
            # Match Objective
            if self.current_phase_id and line.startswith('**Objectif:**'):
                self.phases[self.current_phase_id]['objective'] = line.replace('**Objectif:**', '').strip()
                continue
            
            # Match Dependencies
            if self.current_phase_id and line.startswith('**Dependances:**'):
                self.phases[self.current_phase_id]['dependencies'] = line.replace('**Dependances:**', '').strip()
                continue
            
            # Match Criteria (etapes detaillees section ends, criteria starts)
            if self.current_phase_id and line.startswith('#### Critere d\'acceptation'):
                # Collect criteria lines until next section
                criteria_lines = []
                for j in range(i+1, len(lines)):
                    if lines[j].startswith('####') or lines[j].startswith('---') or lines[j].startswith('###'):
                        break
                    if lines[j].strip():
                        criteria_lines.append(lines[j])
                self.phases[self.current_phase_id]['criteria'] = '\n'.join(criteria_lines).strip()
                continue
            
            # Match Steps table
            if self.current_phase_id and line.startswith('| ID | Etape |'):
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
    
    def generate_issue_title(self, phase_id: str, step: Dict) -> str:
        """Génère le titre de l'issue"""
        return f"{phase_id}: {step['name']}"
    
    def generate_issue_body(self, phase_id: str, step: Dict) -> str:
        """Génère le corps de l'issue"""
        phase = self.phases[phase_id]
        
        body = f"""# {phase_id}: {step['name']}

## Phase
**{phase_id}: {phase['name']}**

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

## Objectif de la Phase
{phase['objective']}

## Critères d'Acceptation
{phase['criteria']}

---

### Dépendances de la Phase
{phase['dependencies']}

### Labels
phase-{phase_id}
"""
        return body
    
    def create_github_issue(self, title: str, body: str, labels: List[str]):
        """Crée une issue GitHub via gh CLI"""
        # Escape newlines for shell
        body_escaped = body.replace('"', '\\"').replace('$', '\\$').replace('`', '\\`')
        label_str = ','.join(labels)
        
        cmd = [
            'gh', 'issue', 'create',
            '--title', title,
            '--body', body,
            '--label', label_str
        ]
        
        print(f"Creating issue: {title}")
        print(f"Command: {' '.join(cmd)}")
        
        try:
            result = subprocess.run(cmd, capture_output=True, text=True, check=False)
            if result.returncode == 0:
                print(f"✓ Issue créée: {title}")
                # Extract issue number from output
                match = re.search(r'#(\d+)', result.stdout)
                if match:
                    print(f"  Issue #{match.group(1)}")
                return True
            else:
                print(f"✗ Erreur: {result.stderr}")
                return False
        except Exception as e:
            print(f"✗ Exception: {e}")
            return False
    
    def run(self, dry_run: bool = True):
        """Exécute la génération des issues"""
        self.parse_roadmap()
        
        print(f"Phases trouvées: {list(self.phases.keys())}")
        
        for phase_id, phase in self.phases.items():
            print(f"\n=== Phase {phase_id}: {phase['name']} ===")
            print(f"Objectif: {phase['objective']}")
            print(f"Étapes: {len(phase['steps'])}")
            
            for step in phase['steps']:
                title = self.generate_issue_title(phase_id, step)
                body = self.generate_issue_body(phase_id, step)
                labels = [f"phase-{phase_id}"]
                
                # Add priority label
                priority = step['priority'].lower()
                if 'critique' in priority:
                    labels.append('priority:critical')
                elif 'haut' in priority or 'high' in priority:
                    labels.append('priority:high')
                elif 'moyen' in priority or 'medium' in priority:
                    labels.append('priority:medium')
                else:
                    labels.append('priority:low')
                
                if not dry_run:
                    self.create_github_issue(title, body, labels)
                else:
                    print(f"\n[DRY RUN] Issue: {title}")
                    print(f"  Labels: {labels}")
                    # print first 200 chars of body
                    print(f"  Body preview: {body[:200]}...")


if __name__ == '__main__':
    dry_run = '--dry-run' in sys.argv
    
    generator = IssueGenerator('/Users/amaury/Documents/REBUILD APPS/Note-Craft/docs/roadmap/roadmap_v3.md')
    generator.run(dry_run=dry_run)
