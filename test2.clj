(ns data-api
"Useful utilities for launching boxes.
  Aidmed to develop and debug Search and Data API."
(:require
[box.core]
[clojure.java.io :as io]
[clojure.string :as str]
[user])
(:import (java.io File)))

;;(def r4 {:url "https://github.com/zen-lang/fhir/releases/latest/download/hl7-fhir-r4-core.zip"
;;         :import "hl7-fhir-r4-core"})
;;
;;(def r5 {:url "https://github.com/zen-lang/fhir/releases/latest/download/hl7-fhir-r5-core.zip"
;;         :import "hl7-fhir-r5-core"})
;;
;;(def r4-http {:http-git-url "https://github.com/zen-fhir/hl7-fhir-r4-core.git"
;;              :import "hl7-fhir-r4-core"})
;;
;;(def r4b-http {:http-git-url "https://github.com/zen-fhir/hl7-fhir-r4b-core.git"
;;               :import "hl7-fhir-r4b-core"})
;;
;;(def r5-http {:http-git-url "https://github.com/zen-fhir/hl7-fhir-r5-core.git"
;;              :import "hl7-fhir-r5-core"})
;; IGs
(def kbv-basis {:http-git-url "https://github.com/zen-fhir/kbv-basis.git"
:import "kbv-basis"})
(foo
;; hello
bar)
