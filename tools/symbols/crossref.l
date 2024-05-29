;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; cross reference
;;;
(crux:intern (crux:find-ns "prelude") "crossref"
   (:lambda (file-name)
      ((:lambda (stream)
          (prelude:mapc
           (:lambda (symbol)
              (:if (crux:boundp symbol)
                   (prelude:format stream "~A~T~A~T~S~%"
                                `(,(crux:symbol-name symbol)
                                  ,(prelude:type-of (crux:symbol-value symbol))
                                  ,(crux:symbol-value symbol)))
                   (prelude:format stream "unbound~Tunbound~T~A~%" `(,(crux:symbol-name symbol)))))
           (crux:symbols (crux:find-ns "prelude"))))
       (crux:open :file :output file-name))
      :t))
