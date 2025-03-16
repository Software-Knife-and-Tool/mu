;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; symbol reference
;;;
(mu:intern mu:*null/* "reference"
   (:lambda (ns file-name)
      ((:lambda (stream)
          (core:%mapc
           (:lambda (symbol)
              (:if (mu:boundp symbol)
                   (core:%format stream "~A~T~A~T~S~T~A~%"
                                `(,(mu:symbol-name symbol)
                                   ,(core:type-of (mu:symbol-value symbol))
                                   ,(mu:symbol-value symbol)
                                   ,(mu:repr :vector symbol)))
                   (core:%format stream "unbound~Tunbound~T~A~Tunbound~%" `(,(mu:symbol-name symbol)))))
           (mu:namespace-symbols (mu:find-namespace ns)))
         (mu:close stream))
       (mu:open :file :output file-name :t))
      :t))
