;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; symbol reference
;;;
(mu:make-namespace "symbols")
((:lambda (symbols)
   (mu:intern symbols "reference"
      (:lambda (ns file-name)
        ((:lambda (stream)
           (core:%mapc
            (:lambda (symbol)
              (:if (mu:boundp symbol)
                   (core:%format stream "~A~T~A~T~S~T~A~%"
                                 `(,(mu:symbol-name symbol)
                                   ,(core:type-of (mu:symbol-value symbol))
                                    ,(mu:symbol-value symbol)
                                    ,(mu:repr symbol)))
                   (core:%format stream "unbound~Tunbound~T~A~Tunbound~%" `(,(mu:symbol-name symbol)))))
            (feature/core:ns-symbols (mu:find-namespace ns)))
           (mu:close stream))
         (mu:open :file :output file-name :t))
        :t)))
 (mu:find-namespace "symbols"))
