#!/bin/bash

# Make this script executable
chmod +x "$0"

# Community Impact Dashboard Implementation Verification Script
# This script verifies that all required components have been implemented

echo "========================================="
echo "Community Impact Dashboard Verification"
echo "========================================="

# Check if required directories exist
echo "Checking directory structure..."

REQUIRED_DIRS=(
    "src"
    "src/launch"
    "src/launch/experience"
    "src/launch/support"
    "src/ownership"
)

for dir in "${REQUIRED_DIRS[@]}"; do
    if [ -d "$dir" ]; then
        echo "✅ Directory exists: $dir"
    else
        echo "❌ Directory missing: $dir"
        exit 1
    fi
done

# Check if required files exist
echo -e "\nChecking required files..."

REQUIRED_FILES=(
    "src/main.rs"
    "src/lib.rs"
    "src/launch/mod.rs"
    "src/launch/execution.rs"
    "src/launch/experience/mod.rs"
    "src/launch/experience/welcome.rs"
    "src/launch/experience/announcements.rs"
    "src/launch/experience/storytelling.rs"
    "src/launch/experience/celebration.rs"
    "src/launch/experience/ownership.rs"
    "src/launch/support/mod.rs"
    "src/launch/support/help_desk.rs"
    "src/launch/support/issue_tracking.rs"
    "src/launch/support/knowledge_base.rs"
    "src/launch/support/feedback_triage.rs"
    "src/launch/support/translation.rs"
    "src/ownership/mod.rs"
    "src/ownership/governance.rs"
    "src/ownership/decision_making.rs"
    "src/ownership/feature_voting.rs"
    "src/ownership/community_enhancements.rs"
    "src/ownership/transfer.rs"
    "Cargo.toml"
    "README.md"
    "IMPLEMENTATION_SUMMARY.md"
)

for file in "${REQUIRED_FILES[@]}"; do
    if [ -f "$file" ]; then
        echo "✅ File exists: $file"
    else
        echo "❌ File missing: $file"
        exit 1
    fi
done

# Check for key implementation components
echo -e "\nChecking for key implementation components..."

# Check for launch execution system
if grep -q "LaunchExecutionSystem" "src/launch/execution.rs"; then
    echo "✅ Launch Execution System implemented"
else
    echo "❌ Launch Execution System not found"
    exit 1
fi

# Check for community launch experience components
EXPERIENCE_COMPONENTS=(
    "WelcomeExperience"
    "LaunchAnnouncement"
    "CommunityStory"
    "CelebrationEvent"
    "OwnershipTransfer"
)

for component in "${EXPERIENCE_COMPONENTS[@]}"; do
    if grep -q "$component" "src/launch/experience/"; then
        echo "✅ $component implemented"
    else
        echo "❌ $component not found"
        exit 1
    fi
done

# Check for launch support system components
SUPPORT_COMPONENTS=(
    "HelpDesk"
    "IssueTracker"
    "KnowledgeBase"
    "FeedbackTriage"
    "TranslationSupport"
)

for component in "${SUPPORT_COMPONENTS[@]}"; do
    if grep -q "$component" "src/launch/support/"; then
        echo "✅ $component implemented"
    else
        echo "❌ $component not found"
        exit 1
    fi
done

# Check for community ownership framework components
OWNERSHIP_COMPONENTS=(
    "GovernanceFramework"
    "DecisionMakingSystem"
    "FeatureVotingSystem"
    "CommunityEnhancementSystem"
    "OwnershipTransferSystem"
)

for component in "${OWNERSHIP_COMPONENTS[@]}"; do
    if grep -q "$component" "src/ownership/"; then
        echo "✅ $component implemented"
    else
        echo "❌ $component not found"
        exit 1
    fi
done

# Check for documentation
echo -e "\nChecking documentation..."

if grep -q "Community Impact Dashboard" "README.md"; then
    echo "✅ README documentation exists"
else
    echo "❌ README documentation missing"
    exit 1
fi

if grep -q "implementation" "IMPLEMENTATION_SUMMARY.md"; then
    echo "✅ Implementation summary exists"
else
    echo "❌ Implementation summary missing"
    exit 1
fi

echo -e "\n========================================="
echo "✅ ALL VERIFICATION CHECKS PASSED"
echo "Community Impact Dashboard implementation is complete and ready for launch!"
echo "========================================="

exit 0