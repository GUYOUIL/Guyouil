use std::error::Error;

use crate::{
    ast::{Identifier, MemberAccess},
    context::loader::{ASTNode, ContextLoader},
    detector::detector::{Detector, IssueSeverity},
    visitor::ast_visitor::{ASTConstVisitor, Node},
};
use eyre::Result;

#[derive(Default)]
pub struct DeprecatedOZFunctionsDetector {
    pub found_deprecated_oz_functions: Vec<Option<ASTNode>>,
}

impl ASTConstVisitor for DeprecatedOZFunctionsDetector {
    fn visit_identifier(&mut self, node: &Identifier) -> Result<bool> {
        if node.name == "_setupRole" {
            self.found_deprecated_oz_functions
                .push(Some(ASTNode::Identifier(node.clone())));
        }
        Ok(true)
    }

    fn visit_member_access(&mut self, node: &MemberAccess) -> Result<bool> {
        if node.member_name == "safeApprove" {
            self.found_deprecated_oz_functions
                .push(Some(ASTNode::MemberAccess(node.clone())));
        }
        Ok(true)
    }
}

impl Detector for DeprecatedOZFunctionsDetector {
    fn detect(&mut self, loader: &ContextLoader) -> Result<bool, Box<dyn Error>> {
        for identifier in loader.get_identifiers() {
            identifier.accept(self)?;
        }
        for member_access in loader.get_member_accesses() {
            member_access.accept(self)?;
        }
        Ok(self.found_deprecated_oz_functions.len() > 0)
    }

    fn title(&self) -> String {
        String::from("Deprecated OpenZeppelin functions should not be used")
    }

    fn description(&self) -> String {
        String::from("Openzeppelin has deprecated several functions and replaced with newer versions. Please consult https://docs.openzeppelin.com/")
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::Low
    }

    fn instances(&self) -> Vec<Option<ASTNode>> {
        self.found_deprecated_oz_functions.clone()
    }
}

#[cfg(test)]
mod deprecated_oz_functions_tests {
    use crate::detector::detector::{detector_test_helpers::load_contract, Detector};

    use super::DeprecatedOZFunctionsDetector;

    #[test]
    fn test_deprecated_oz_functions_detector() {
        let context_loader = load_contract(
            "./tests/contract-playground/out/DeprecatedOZFunctions.sol/DeprecatedOZFunctions.json",
        );
        let mut detector = DeprecatedOZFunctionsDetector::default();
        let found = detector.detect(&context_loader).unwrap();
        // assert that the detector found an abi encode packed
        assert!(found);
        // assert that the detector found the correct abi encode packed
        // failure0, failure1 and failure3
        assert_eq!(detector.instances().len(), 2);
        // assert that the severity is low
        assert_eq!(
            detector.severity(),
            crate::detector::detector::IssueSeverity::Low
        );
        // assert that the title is correct
        assert_eq!(
            detector.title(),
            String::from("Deprecated OpenZeppelin functions should not be used")
        );
        // assert that the description is correct
        assert_eq!(
            detector.description(),
            String::from(
                "Openzeppelin has deprecated several functions and replaced with newer versions. Please consult https://docs.openzeppelin.com/"
            )
        );
    }
}
